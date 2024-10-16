use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

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
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: PreviousEntry,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = PreviousEntry;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.payload)
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
