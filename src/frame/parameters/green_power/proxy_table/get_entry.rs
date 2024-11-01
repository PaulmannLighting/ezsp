//! Parameters for the [`GreenPower::proxy_table_get_entry`](crate::GreenPower::proxy_table_get_entry) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::gp::proxy::TableEntry;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00C8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    proxy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(proxy_index: u8) -> Self {
        Self { proxy_index }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    entry: TableEntry,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into a [`TableEntry`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for TableEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.entry),
            other => Err(other.into()),
        }
    }
}
