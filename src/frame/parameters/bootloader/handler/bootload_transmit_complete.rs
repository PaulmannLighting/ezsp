use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

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
        match Status::try_from(self.status) {
            Ok(Status::Success) => Ok(true),
            Ok(Status::DeliveryFailed) => Ok(false),
            Ok(status) => Err(Error::Ember(status)),
            Err(error) => Err(ValueError::Ember(error).into()),
        }
    }

    /// The message that was sent.
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
