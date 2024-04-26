pub mod incoming_message;
pub mod sent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    IncomingMessage(incoming_message::Handler),
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
