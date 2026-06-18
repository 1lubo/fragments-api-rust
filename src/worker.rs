//! The consumer worker — the heart of the ingest loop.
//!
//! Java/Spring parallel: this plays the role of a Kafka listener's
//! consume-then-store flow. A `@KafkaListener` thread pulls a record, forwards
//! it downstream, then stores it with the dispatch result. Here the "listener
//! thread" is a `tokio` task draining an `mpsc` channel.
//!
//! Rust lessons packed into this small file:
//!   * **Generics over traits** (`D: Dispatcher`, `S: FragmentStore`) — static
//!     dispatch, the moral equivalent of injecting interfaces in Spring, but
//!     resolved at compile time (like Java generics, no `dyn`/boxing).
//!   * **`tokio::mpsc`** — the receiver half is our consumer poll loop; the
//!     sender half (held elsewhere) is the producer.
//!   * **`Arc<Mutex<…>>`** — the same shared store the HTTP handlers use, so a
//!     `GET /fragments` sees whatever the worker just inserted.

use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::Receiver;

use crate::dispatcher::Dispatcher;
use crate::message::ConsumedMessage;
use crate::model::{Fragment, MessageType};
use crate::repository::FragmentStore;

/// Java/Spring: ≈ the consumer bean. Owns its inbound channel, the dispatcher,
/// and a handle to the shared store.
pub struct Worker<D, S> {
    rx: Receiver<ConsumedMessage>,
    dispatcher: D,
    store: Arc<Mutex<S>>,
}

impl<D, S> Worker<D, S>
where
    D: Dispatcher,
    S: FragmentStore,
{
    /// Wire the worker up. The `Receiver` is the consuming end of the topic; the
    /// `store` is shared (cloned `Arc`) with the HTTP layer.
    pub fn new(rx: Receiver<ConsumedMessage>, dispatcher: D, store: Arc<Mutex<S>>) -> Self {
        Self {
            rx,
            dispatcher,
            store,
        }
    }

    /// Run until the channel closes. `recv().await` yields `None` once every
    /// `Sender` has been dropped — that is our clean shutdown signal (≈ the
    /// listener container stopping). Takes `self` by value so the worker (and its
    /// receiver) is consumed by the task that runs it.
    pub async fn run(mut self) {
        while let Some(message) = self.rx.recv().await {
            self.process(message);
        }
    }

    /// Process one record: derive the verb, dispatch downstream, persist the
    /// outcome — dispatch first, then store the returned status.
    ///
    /// TODO(milestone A): derive `MessageType` from `message.value`
    /// (`Some` ⇒ Update, `None` ⇒ Delete), call `self.dispatcher.dispatch(...)`,
    /// build a `Fragment::with_status(...)` (deletes persist an empty payload),
    /// then `self.store.lock().unwrap().insert(fragment)`.
    ///
    /// Java to port (≈ the body of a `@KafkaListener` method):
    /// ```java
    /// void process(ConsumerRecord<String, String> record) {
    ///     MessageType type = record.value() != null ? MessageType.UPDATE : MessageType.DELETE;
    ///     FragmentStatus status = dispatcher.dispatch(record.key(), type, record.value());
    ///     String payload = record.value() != null ? record.value() : "";
    ///     Fragment f = Fragment.of(record.key(), type, fragmentType, record.timestamp(), payload, status);
    ///     repository.save(f);
    /// }
    /// ```
    fn process(&self, message: ConsumedMessage) {
        todo!("milestone A: derive verb, dispatch, then store Fragment::with_status")
    }
}
