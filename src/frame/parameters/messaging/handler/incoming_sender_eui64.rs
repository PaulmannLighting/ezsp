use le_stream::derive::FromLeStream;

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x0062;

/// A callback indicating a message has been received containing the EUI64 of the sender.
///
/// This callback is called immediately before the incomingMessageHandler callback.
///
/// It is not called if the incoming message did not contain the EUI64 of the sender.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    sender_eui64: Eui64,
}

impl Handler {
    /// The EUI64 of the sender.
    #[must_use]
    pub const fn sender_eui64(&self) -> Eui64 {
        self.sender_eui64
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}
