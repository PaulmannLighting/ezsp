use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x0098;

/// A callback invoked by the `EmberZNet` stack when the MAC has finished transmitting a raw message.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    /// Returns the status of the raw transmit.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the message was successfully transmitted.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is [`Status::DeliveryFailed`] or is not a valid status.
    pub fn status(&self) -> Result<(), Error> {
        Status::try_from(self.status).resolve()
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
