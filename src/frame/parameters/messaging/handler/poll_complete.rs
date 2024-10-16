use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x0043;

/// Indicates the result of a data poll to the parent of the local node.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    /// Returns the status of the poll.
    ///
    /// # Returns
    /// `Ok(())` if data was received in response to the poll.
    ///
    /// # Errors
    ///
    /// One of the following expected errors:
    ///
    /// - [`Status::Mac(Mac::NoData)`](Mac::NoData) No data was pending.
    /// - [`Status::DeliveryFailed`] The poll message could not be sent.
    /// - [`Status::Mac(Mac::NoAckReceived)`](Mac::NoAckReceived) The poll message was sent but not acknowledged by the parent.
    ///
    /// Returns any other [`Error`] if the value is not a valid status.
    pub fn status(&self) -> Result<(), Error> {
        Status::try_from(self.status).resolve()
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
