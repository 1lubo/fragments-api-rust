//! Step 5 & 6 tests — the HTTP layer, end to end, with no real network.
//!
//! Java/Spring parallel: this is `@WebMvcTest` + `MockMvc`. `tower`'s `oneshot`
//! sends a single `Request` straight into the router and hands you the
//! `Response` — exactly like `mockMvc.perform(...)`, without binding a port.
//! Run with:  `cargo test --test api_tests`

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt; // for `.collect()`
use tower::ServiceExt; // for `.oneshot()`

use fragments_api::api::build_router;
use fragments_api::model::Fragment;
use fragments_api::state::new_shared_state;

fn app() -> axum::Router {
    build_router(new_shared_state())
}

async fn body_string(resp: axum::response::Response) -> String {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

const CREATE_BODY: &str = r#"{
    "id": "id-1",
    "messageType": "UPDATE",
    "fragmentType": "movie",
    "messageTs": 100,
    "fragment": "payload"
}"#;

#[tokio::test]
async fn healthz_returns_200() {
    let resp = app()
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn list_is_empty_initially() {
    let resp = app()
        .oneshot(Request::get("/fragments").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(body_string(resp).await, "[]");
}

#[tokio::test]
async fn get_missing_fragment_returns_404() {
    let resp = app()
        .oneshot(Request::get("/fragments/nope").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_returns_201_and_persists() {
    // Share one router (one state) across multiple requests.
    let app = app();

    let created = app
        .clone()
        .oneshot(
            Request::post("/fragments")
                .header("content-type", "application/json")
                .body(Body::from(CREATE_BODY))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::CREATED);

    let fragment: Fragment = serde_json::from_str(&body_string(created).await).unwrap();
    assert_eq!(fragment.id, "id-1");
    // Server assigns QUEUED regardless of input.
    assert_eq!(fragment.status, fragments_api::model::FragmentStatus::Queued);

    // Now GET it back.
    let fetched = app
        .clone()
        .oneshot(Request::get("/fragments/id-1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(fetched.status(), StatusCode::OK);

    // And it shows up in the list.
    let listed = app
        .oneshot(Request::get("/fragments").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let list: Vec<Fragment> = serde_json::from_str(&body_string(listed).await).unwrap();
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn delete_existing_returns_204_then_404() {
    let app = app();

    app.clone()
        .oneshot(
            Request::post("/fragments")
                .header("content-type", "application/json")
                .body(Body::from(CREATE_BODY))
                .unwrap(),
        )
        .await
        .unwrap();

    let deleted = app
        .clone()
        .oneshot(
            Request::delete("/fragments/id-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(deleted.status(), StatusCode::NO_CONTENT);

    let deleted_again = app
        .oneshot(
            Request::delete("/fragments/id-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(deleted_again.status(), StatusCode::NOT_FOUND);
}
