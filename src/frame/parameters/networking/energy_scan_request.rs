use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x009C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
