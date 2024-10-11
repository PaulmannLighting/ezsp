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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    old_eui64: Eui64,
    old_id: NodeId,
    old_extended_timeout: bool,
}

impl Payload {
    #[must_use]
    pub const fn old_eui64(&self) -> Eui64 {
        self.old_eui64
    }

    #[must_use]
    pub const fn old_id(&self) -> NodeId {
        self.old_id
    }

    #[must_use]
    pub const fn old_extended_timeout(&self) -> bool {
        self.old_extended_timeout
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: Payload,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Payload;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.payload)
    }
}
