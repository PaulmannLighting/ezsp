pub mod rx;

/// Handler of a MFGlib event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Received data event.
    Rx(rx::Handler),
}

impl From<rx::Handler> for Handler {
    fn from(handler: rx::Handler) -> Self {
        Self::Rx(handler)
    }
}
