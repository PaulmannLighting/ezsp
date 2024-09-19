use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x009C;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    target: NodeId,
    scan_channels: u32,
    scan_duration: u8,
    scan_count: u16,
}

impl Command {
    #[must_use]
    pub const fn new(
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> Self {
        Self {
            target,
            scan_channels,
            scan_duration,
            scan_count,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
