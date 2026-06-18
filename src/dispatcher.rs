//! Downstream dispatch — the "forward it on" step.
//!
//! Java/Spring parallel: this is the "forward it on" step. In a real consumer
//! it makes an HTTP/Kafka call to a downstream service and maps the outcome to a
//! `FragmentStatus`. Crucially it does NOT throw on a downstream failure — it
//! *returns a status* so the caller can persist it.
//!
//! `Dispatcher` is the seam (≈ a Spring `@Service` interface). Milestone A ships
//! an in-memory `FakeDispatcher`; Milestone B swaps in a real client without the
//! worker ever changing — that swap is the dependency-inversion lesson.

use crate::model::{FragmentStatus, MessageType};

/// Java/Spring: the dispatcher interface. `&self` because dispatching shouldn't
/// need to mutate the dispatcher itself.
///
/// We take the value as `Option<&str>` to mirror UPDATE (`Some`) vs DELETE
/// (`None`), and return the `FragmentStatus` to persist.
pub trait Dispatcher {
    fn dispatch(&self, id: &str, message_type: MessageType, value: Option<&str>) -> FragmentStatus;
}

/// Java/Spring: a Mockito-style stub. Configure the status it should return so
/// tests can drive every branch of the consume→store flow deterministically.
#[derive(Debug, Clone)]
pub struct FakeDispatcher {
    status: FragmentStatus,
}

impl FakeDispatcher {
    /// A dispatcher that always reports success (`Processed`).
    pub fn always_ok() -> Self {
        Self {
            status: FragmentStatus::Processed,
        }
    }

    /// A dispatcher that always reports the given status — handy for exercising
    /// the downstream-error branches.
    pub fn always(status: FragmentStatus) -> Self {
        Self { status }
    }
}

impl Dispatcher for FakeDispatcher {
    fn dispatch(
        &self,
        _id: &str,
        _message_type: MessageType,
        _value: Option<&str>,
    ) -> FragmentStatus {
        self.status
    }
}
