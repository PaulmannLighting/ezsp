//! Indicates that the NCP received an invalid command.

use core::fmt::Display;

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::Status;
use crate::frame::Parameter;

const ID: u16 = 0x0058;

/// Response sent by the NCP when it received an invalid command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    reason: u8,
}

impl Response {
    /// The unique ID of the [`Response`] frame type.
    pub const ID: u16 = ID;

    /// Returns the reason for the invalid command response.
    ///
    /// # Errors
    ///
    /// Returns an error if the reason is not a valid [`Status`].
    pub fn reason(&self) -> Result<Status, u8> {
        Status::from_u8(self.reason).ok_or(self.reason)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason() {
            Ok(status) => write!(f, "{status} ({status:#04X})"),
            Err(reason) => write!(f, "Unknown reason: {reason:#04X}"),
        }
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
