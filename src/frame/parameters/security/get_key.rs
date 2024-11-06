//! Parameters for the [`Security::get_key`](crate::Security::get_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::key::{Struct, Type};
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x006A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    key: u8,
}

impl Command {
    /// Creates a new `Command`.
    #[must_use]
    pub const fn new(key: Type) -> Self {
        Self { key: key as u8 }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    key: Struct,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for Struct {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.key),
            other => Err(other.into()),
        }
    }
}
