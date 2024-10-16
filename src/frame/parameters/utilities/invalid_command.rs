//! Indicates that the NCP received an invalid command.

use crate::ezsp::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0058;

/// Response sent by the NCP when it received an invalid command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    reason: u8,
}

impl Response {
    /// Returns the reason for the invalid command response.
    ///
    /// # Errors
    ///
    /// Returns an error if the reason is not a valid [`Status`].
    pub fn reason(&self) -> Result<Status, u8> {
        Status::try_from(self.reason)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
