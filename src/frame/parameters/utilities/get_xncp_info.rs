use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0013;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: Option<Payload>,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for Payload {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => response
                .payload
                .ok_or_else(|| ValueError::MissingPayload.into()),
            other => Err(other.into()),
        }
    }
}

/// Payload of the get XNCP info command.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    manufacturer_id: u16,
    version_number: u16,
}

impl Payload {
    /// Returns the manufacturer ID.
    #[must_use]
    pub const fn manufacturer_id(self) -> u16 {
        self.manufacturer_id
    }

    /// Returns the version number.
    #[must_use]
    pub const fn version_number(self) -> u16 {
        self.version_number
    }
}
