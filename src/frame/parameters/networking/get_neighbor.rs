//! Parameters for the [`Networking::get_neighbor`](crate::Networking::get_neighbor) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::neighbor::TableEntry;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0079;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    value: TableEntry,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert a response into a [`TableEntry`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for TableEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.value),
            other => Err(other.into()),
        }
    }
}
