//! `Mfglib` event handler.

mod rx;

pub use rx::Handler as Rx;

/// Handler of a `Mfglib` event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Received data event.
    Rx(Rx),
}
