//! Application errors.
//!
//! Java/Spring parallel: this combines two Spring ideas:
//!   * a custom exception type (e.g. `ResourceNotFoundException`), and
//!   * a `@ControllerAdvice` / `@ExceptionHandler` that turns that exception
//!     into an HTTP response with the right status code.
//!
//! In axum, returning a `Result<T, AppError>` from a handler and implementing
//! `IntoResponse` for `AppError` *is* the exception-handler mechanism.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

/// Java/Spring: your domain exceptions. `thiserror`'s `#[error("...")]` is like
/// the message you'd pass to `new RuntimeException("...")`.
#[derive(Debug, Error)]
pub enum AppError {
    /// Java/Spring: maps to `404 NOT_FOUND` (≈ `ResponseStatusException(NOT_FOUND)`).
    #[error("fragment not found: {0}")]
    NotFound(String),

    /// Java/Spring: maps to `400 BAD_REQUEST`.
    #[error("bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    /// Java/Spring: the body of your `@ExceptionHandler` — pick a status code and
    /// build the response body.
    ///
    /// TODO(step 6): map each variant to its `StatusCode`, then return
    /// `(status, Json(json!({ "error": message }))).into_response()`.
    /// Hint: `self.to_string()` gives you the `#[error("...")]` message.
    ///
    /// Java to port (≈ a `@ControllerAdvice` `@ExceptionHandler`):
    /// ```java
    /// @ExceptionHandler(AppException.class)
    /// ResponseEntity<Map<String, String>> handle(AppException ex) {
    ///     HttpStatus status = switch (ex.kind()) {
    ///         case NOT_FOUND   -> HttpStatus.NOT_FOUND;
    ///         case BAD_REQUEST -> HttpStatus.BAD_REQUEST;
    ///     };
    ///     return ResponseEntity.status(status).body(Map.of("error", ex.getMessage()));
    /// }
    /// ```
    fn into_response(self) -> Response {
        todo!("step 6: map AppError variants to (StatusCode, Json body)")
    }
}
