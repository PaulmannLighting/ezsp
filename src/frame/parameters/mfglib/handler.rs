//! `Mfglib` event handler.

pub use self::rx::Handler as Rx;

mod rx;

/// Handler of a `Mfglib` event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Received data event.
    Rx(Rx),
}
