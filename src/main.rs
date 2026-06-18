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
use fragments_api::state::new_shared_state;

#[tokio::main]
async fn main() {
    // Java/Spring: like Logback/`application.yml` logging setup.
    tracing_subscriber::fmt::init();

    // TODO(step 7): wire the HTTP server:
    //   1. let state = new_shared_state();
    //   2. let app = build_router(state.clone());
    //   3. let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    //   4. announce the address Spring-style: tracing::info!("Listening on {}", ...);
    //   5. axum::serve(listener, app).await.unwrap();
    //
    // TODO(milestone A): also spawn the consumer worker beside the server, so the
    // HTTP layer and the worker share ONE store (clone the `Arc`). You'll need to
    // add these imports:
    //   use fragments_api::dispatcher::FakeDispatcher;
    //   use fragments_api::message::ConsumedMessage;
    //   use fragments_api::model::FragmentType;
    //   use fragments_api::worker::Worker;
    // Then:
    //   a. let (tx, rx) = tokio::sync::mpsc::channel(64);   // the "topic"
    //   b. let worker = Worker::new(rx, FakeDispatcher::always_ok(), state.clone());
    //   c. tokio::spawn(worker.run());                      // ≈ @KafkaListener container
    //   d. (optional) send a couple of ConsumedMessage::update/delete on `tx` from a
    //      spawned task so `GET /fragments` shows ingested rows on startup.
    let _ = (new_shared_state, build_router);
    todo!("step 7 + milestone A: wire the HTTP server and the consumer worker")
}
