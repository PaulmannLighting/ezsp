use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0082;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: PreviousEntry,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for PreviousEntry {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.payload)
                } else {
                    Err(status.into())
                }
            })
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
