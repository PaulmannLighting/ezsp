pub mod bootload_transmit_complete;
pub mod incoming_bootload_message;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Handler {
    BootloadTransmitComplete(bootload_transmit_complete::Handler),
    IncomingBootloadMessage(incoming_bootload_message::Handler),
}

impl From<bootload_transmit_complete::Handler> for Handler {
    fn from(handler: bootload_transmit_complete::Handler) -> Self {
        Self::BootloadTransmitComplete(handler)
    }
}

impl From<incoming_bootload_message::Handler> for Handler {
    fn from(handler: incoming_bootload_message::Handler) -> Self {
        Self::IncomingBootloadMessage(handler)
    }
}
