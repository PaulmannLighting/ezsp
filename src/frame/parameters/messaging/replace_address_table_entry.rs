//! Parameters for the [`Messaging::replace_address_table_entry`](crate::Messaging::replace_address_table_entry) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;

const ID: u16 = 0x0082;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address_table_index: u8,
    new_eui64: Eui64,
    new_id: NodeId,
    new_extended_timeout: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> Self {
        Self {
            address_table_index,
            new_eui64,
            new_id,
            new_extended_timeout,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: PreviousEntry,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into the [`PreviousEntry`]
/// or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for PreviousEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}

/// Information about the previous entry that was replaced.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct PreviousEntry {
    eui64: Eui64,
    id: NodeId,
    extended_timeout: bool,
}

impl PreviousEntry {
    /// Returns the old EUI64.
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    /// Returns the old node ID.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns if the old entry had an extended timeout.
    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}
