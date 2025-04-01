use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x001C;

/// Returns the status of the current scan of type
/// [`Type::EnergyScan`](crate::ezsp::network::scan::Type::EnergyScan) or
/// [`Type::ActiveScan`](crate::ezsp::network::scan::Type::ActiveScan).
///
/// [`Status::Success`] signals that the scan has completed.
///
/// Other error conditions signify a failure to scan on the channel specified.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    channel: u8,
    status: u8,
}

impl Handler {
    /// The channel on which the current error occurred.
    ///
    /// Undefined for the case of [`Status::Success`].
    #[must_use]
    pub fn channel(&self) -> Option<u8> {
        match self.status() {
            Ok(()) => None,
            Err(_) => Some(self.channel),
        }
    }

    /// The error condition that occurred on the current channel.
    ///
    /// Value will be [`Status::Success`] when the scan has completed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is invalid.
    pub fn status(&self) -> Result<(), Error> {
        match Status::from_u8(self.status).ok_or(self.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
