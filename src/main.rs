//! Binary entrypoint.
//!
//! Java/Spring parallel: this is your `@SpringBootApplication` class with the
//! `public static void main`. Spring Boot auto-configures an embedded Tomcat and
//! starts listening; here you do the equivalent by hand: build the router, bind
//! a TCP listener, and serve.
//!
//! `#[tokio::main]` sets up the async runtime (≈ the servlet container's thread
//! pool) so `main` can be `async`.

use fragments_api::api::build_router;
use fragments_api::dispatcher::FakeDispatcher;
use fragments_api::message::ConsumedMessage;
use fragments_api::model::FragmentType;
use fragments_api::state::new_shared_state;
use fragments_api::worker::Worker;

#[tokio::main]
async fn main() {
    // Java/Spring: like Logback/`application.yml` logging setup.
    tracing_subscriber::fmt::init();

    // One shared store, used by BOTH the HTTP layer and the consumer worker —
    // cloning the `Arc` just shares the same repository (≈ a singleton bean).
    let state = new_shared_state();

    // The "topic": producers send on `tx`, the worker consumes from `rx`.
    let (tx, rx) = tokio::sync::mpsc::channel(64);

    // Spawn the consumer beside the server (≈ the @KafkaListener container).
    let worker = Worker::new(rx, FakeDispatcher::always_ok(), state.clone());
    tokio::spawn(worker.run());

    // Tiny demo producer so `GET /fragments` shows ingested rows on startup.
    let demo_tx = tx.clone();
    tokio::spawn(async move {
        let _ = demo_tx
            .send(ConsumedMessage::update(
                "demo-1",
                FragmentType::Movie,
                1,
                r#"{"title":"hello"}"#,
            ))
            .await;
        let _ = demo_tx
            .send(ConsumedMessage::delete("demo-2", FragmentType::Series, 2))
            .await;
    });

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
