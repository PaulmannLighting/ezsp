//! Green Power cluster handler.

pub use self::incoming_message::{Handler as IncomingMessage, Payload};
pub use self::sent::Handler as Sent;

mod incoming_message;
mod sent;

/// Callbacks for the Green Power cluster.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    /// Callbacks for incoming messages.
    IncomingMessage(IncomingMessage),
    /// Callbacks for sent messages.
    Sent(Sent),
}
