//! Green Power cluster handler.

mod incoming_message;
mod sent;

pub use incoming_message::Handler as IncomingMessage;
pub use incoming_message::Payload;
pub use sent::Handler as Sent;

/// Callbacks for the Green Power cluster.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Callbacks for incoming messages.
    IncomingMessage(IncomingMessage),
    /// Callbacks for sent messages.
    Sent(Sent),
}
