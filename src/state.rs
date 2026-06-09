//! Shared application state.
//!
//! Java/Spring parallel: in Spring you inject a *singleton* `@Repository` bean
//! into every controller; the framework hands every request the same instance.
//!
//! Rust has no container doing that for you, so you make sharing explicit:
//!   * `Arc<T>`  = atomically reference-counted pointer → lets many handlers
//!                 (threads/tasks) share ownership of one repository.
//!   * `Mutex<T>` = guards mutation so two requests can't corrupt the map at
//!                 once (Spring relies on the DB / thread-safety for this).
//!
//! Together, `Arc<Mutex<FragmentRepository>>` ≈ "one shared, thread-safe
//! repository bean".

use std::sync::{Arc, Mutex};

use crate::repository::FragmentRepository;

/// The handle every axum handler will receive (axum clones it per request —
/// cloning an `Arc` just bumps the refcount, it does NOT copy the repository).
pub type SharedState = Arc<Mutex<FragmentRepository>>;

/// Java/Spring: this is the "bean definition" — it constructs the single
/// repository and wraps it for sharing.
///
/// TODO(step 4): create a `FragmentRepository` and wrap it in `Arc::new(Mutex::new(...))`.
pub fn new_shared_state() -> SharedState {
    todo!("step 4: build Arc<Mutex<FragmentRepository>>")
}
