//! Parameters for the [`Messaging::maximum_payload_length`](crate::Messaging::maximum_payload_length) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0033;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    aps_length: u8,
}

impl Response {
    /// Returns the maximum payload length in bytes.
    #[must_use]
    pub const fn aps_length(&self) -> u8 {
        self.aps_length
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
