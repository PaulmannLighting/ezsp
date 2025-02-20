use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0093;

/// A callback invoked by the `EmberZNet` stack when the MAC
/// has finished transmitting a bootload message.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    /// Returns `true` if an ACK was received from the destination or `false` if no ACK was received.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is not [`Status::Success`] or [`Status::DeliveryFailed`].
    pub fn ack_received(&self) -> Result<bool, Error> {
        match Status::from_u8(self.status).ok_or(self.status) {
            Ok(Status::Success) => Ok(true),
            Ok(Status::DeliveryFailed) => Ok(false),
            other => Err(other.into()),
        }
    }

    /// The message that was sent.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
