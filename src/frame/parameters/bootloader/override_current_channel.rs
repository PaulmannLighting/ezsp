//! Paramters for the [`Bootloader::override_current_channel()`](crate::Bootloader::override_current_channel) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::disambiguation::Disambiguation;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0095;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    channel: u8,
}

impl Command {
    #[must_use]
    pub const fn new(channel: u8) -> Self {
        Self { channel }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
    const DISAMBIGUATION: Disambiguation = Disambiguation::OverrideCurrentChannel;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
    const DISAMBIGUATION: Disambiguation = Disambiguation::OverrideCurrentChannel;
}

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
