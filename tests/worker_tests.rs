//! Milestone A tests — the Kafka-style consumer worker, end to end, no network.
//!
//! Java/Spring parallel: think `@SpringBootTest` exercising a `@KafkaListener`
//! against an embedded broker — except our "broker" is a `tokio::mpsc` channel
//! and the downstream is a `FakeDispatcher` stub (≈ Mockito). We push records,
//! close the channel to signal shutdown, await the task, then assert on the
//! shared store.
//! Run with:  `cargo test --test worker_tests`

use fragments_api::dispatcher::FakeDispatcher;
use fragments_api::message::ConsumedMessage;
use fragments_api::model::{FragmentStatus, FragmentType, MessageType};
use fragments_api::state::{new_shared_state, SharedState};
use fragments_api::worker::Worker;

use tokio::sync::mpsc;

/// Spin up a worker on a fresh shared store and return the sender + store so the
/// test can produce messages and inspect the result.
fn spawn_worker(
    dispatcher: FakeDispatcher,
) -> (mpsc::Sender<ConsumedMessage>, SharedState, tokio::task::JoinHandle<()>) {
    let state = new_shared_state();
    let (tx, rx) = mpsc::channel(16);
    let worker = Worker::new(rx, dispatcher, state.clone());
    let handle = tokio::spawn(worker.run());
    (tx, state, handle)
}

#[tokio::test]
async fn update_message_is_dispatched_and_stored_with_processed_status() {
    let (tx, state, handle) = spawn_worker(FakeDispatcher::always_ok());

    tx.send(ConsumedMessage::update(
        "id-1",
        FragmentType::Movie,
        100,
        "payload",
    ))
    .await
    .unwrap();

    drop(tx); // close the channel → worker drains and exits
    handle.await.unwrap();

    let stored = state.lock().unwrap().get("id-1").expect("fragment stored");
    assert_eq!(stored.message_type, MessageType::Update);
    assert_eq!(stored.status, FragmentStatus::Processed);
    assert_eq!(stored.fragment, "payload");
}

#[tokio::test]
async fn failed_dispatch_stores_downstream_error_status() {
    let (tx, state, handle) = spawn_worker(FakeDispatcher::always(FragmentStatus::DownstreamError));

    tx.send(ConsumedMessage::update(
        "id-2",
        FragmentType::Series,
        200,
        "payload",
    ))
    .await
    .unwrap();

    drop(tx);
    handle.await.unwrap();

    let stored = state.lock().unwrap().get("id-2").expect("fragment stored");
    // The *dispatch outcome* is what gets persisted, not a blanket "Queued".
    assert_eq!(stored.status, FragmentStatus::DownstreamError);
}

#[tokio::test]
async fn delete_message_records_a_delete_event() {
    let (tx, state, handle) = spawn_worker(FakeDispatcher::always_ok());

    tx.send(ConsumedMessage::delete("id-3", FragmentType::Person, 300))
        .await
        .unwrap();

    drop(tx);
    handle.await.unwrap();

    let stored = state.lock().unwrap().get("id-3").expect("delete recorded");
    assert_eq!(stored.message_type, MessageType::Delete);
    assert_eq!(stored.fragment, ""); // tombstone → empty payload
    assert_eq!(stored.status, FragmentStatus::Processed);
}

#[tokio::test]
async fn worker_drains_multiple_messages() {
    let (tx, state, handle) = spawn_worker(FakeDispatcher::always_ok());

    for i in 0..3 {
        tx.send(ConsumedMessage::update(
            format!("id-{i}"),
            FragmentType::Title,
            i as i64,
            "payload",
        ))
        .await
        .unwrap();
    }

    drop(tx);
    handle.await.unwrap();

    assert_eq!(state.lock().unwrap().len(), 3);
}
