//! HTTP layer — handlers + router.
//!
//! Java/Spring parallel: this file is your `@RestController` plus the route
//! mapping. Each `async fn` below is a `@GetMapping` / `@PostMapping` /
//! `@DeleteMapping` method, and `build_router` is where those mappings are
//! declared (Spring does it via annotations; axum does it explicitly).
//!
//! Extractors (the function arguments) replace Spring's parameter annotations:
//!   * `State(state)`          ≈ the injected `@Autowired` repository bean
//!   * `Path(id)`              ≈ `@PathVariable String id`
//!   * `Json(body)`            ≈ `@RequestBody CreateFragment body`
//!     And the return type is the `ResponseEntity`.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

use crate::error::AppError;
use crate::model::{CreateFragment, Fragment};
use crate::state::SharedState;

/// Java/Spring: `GET /healthz` — a liveness probe. No body, just `200 OK`.
///
/// TODO(step 5): return `StatusCode::OK`.
pub async fn health() -> StatusCode {
    StatusCode::OK
}

/// Java/Spring: `@GetMapping("/fragments")` returning `List<Fragment>`.
///
/// TODO(step 5): lock the state, call `list()`, return `Json(Vec<Fragment>)`.
/// Hint: `state.lock().unwrap()` gives you the repository.
pub async fn list_fragments(State(state): State<SharedState>) -> Json<Vec<Fragment>> {
   let fragments = state.lock().unwrap().list();
    Json(fragments)
}

/// Java/Spring: `@GetMapping("/fragments/{id}")`. 404 when missing.
///
/// TODO(step 6): look up `id`; return `Ok(Json(fragment))` or
/// `Err(AppError::NotFound(id))`.
pub async fn get_fragment(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<Fragment>, AppError> {
    let fragment = state.lock().unwrap().get(&id);
    match fragment {
        Some(f) => Ok(Json(f)),
        None => Err(AppError::NotFound(id)),
    }
}

/// Java/Spring: `@PostMapping("/fragments")` returning `201 Created`.
///
/// TODO(step 6): build a `Fragment` from `body` via `Fragment::new(...)`,
/// insert it, and return `(StatusCode::CREATED, Json(fragment))`.
pub async fn create_fragment(
    State(state): State<SharedState>,
    Json(body): Json<CreateFragment>,
) -> Result<(StatusCode, Json<Fragment>), AppError> {
    let fragment = Fragment::new(
        body.id,
        body.message_type,
        body.fragment_type,
        body.message_ts,
        body.fragment,
    );
    state.lock().unwrap().insert(fragment.clone());
    Ok((StatusCode::CREATED, Json(fragment)))
}

/// Java/Spring: `@DeleteMapping("/fragments/{id}")` → 204, or 404 if absent.
///
/// TODO(step 6): delete by `id`; return `StatusCode::NO_CONTENT` on success or
/// `AppError::NotFound(id)` otherwise.
pub async fn delete_fragment(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let deleted = state.lock().unwrap().delete(&id);
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(id))
    }
}

/// Java/Spring: the route table — equivalent to all the `@RequestMapping`
/// annotations across a controller, plus registering the controller bean.
///
/// TODO(step 5): build and return a `Router` that wires:
///   GET    /healthz         -> health
///   GET    /fragments       -> list_fragments
///   GET    /fragments/{id}  -> get_fragment
///   POST   /fragments       -> create_fragment
///   DELETE /fragments/{id}  -> delete_fragment
/// and attaches the shared state with `.with_state(state)`.
///
/// Hint: `Router::new().route("/healthz", get(health))` ... then
/// `.route("/fragments", get(list_fragments).post(create_fragment))`, etc.
pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/fragments", get(list_fragments).post(create_fragment))
        .route("/fragments/{id}", get(get_fragment).delete(delete_fragment))
        .with_state(state)
}
