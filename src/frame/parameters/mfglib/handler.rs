//! MFGlib event handler.

mod rx;

pub use rx::Handler as Rx;

/// Handler of a MFGlib event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Received data event.
    Rx(Rx),
}

impl From<Rx> for Handler {
    fn from(handler: Rx) -> Self {
        Self::Rx(handler)
    }
}
