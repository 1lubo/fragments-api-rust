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

    // TODO(step 7): wire it all together:
    //   1. let state = new_shared_state();
    //   2. let app = build_router(state);
    //   3. bind a listener:
    //        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    //   4. serve:
    //        axum::serve(listener, app).await.unwrap();
    // Add a `tracing::info!` line announcing the bound address, Spring-style.
    let _ = (new_shared_state, build_router);
    let state = new_shared_state();
    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
