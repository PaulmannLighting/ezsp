use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{NodeId, Status};
use crate::frame::Identified;
use crate::Error;

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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
