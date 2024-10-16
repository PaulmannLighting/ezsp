pub mod incoming_message;
pub mod sent;

/// Callbacks for the Green Power cluster.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Callbacks for incoming messages.
    IncomingMessage(incoming_message::Handler),
    /// Callbacks for sent messages.
    Sent(sent::Handler),
}

impl From<incoming_message::Handler> for Handler {
    fn from(handler: incoming_message::Handler) -> Self {
        Self::IncomingMessage(handler)
    }
}

impl From<sent::Handler> for Handler {
    fn from(handler: sent::Handler) -> Self {
        Self::Sent(handler)
    }
}
