use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::Status;

use crate::error::Resolve;
use crate::ezsp::network::scan::Type;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x001A;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    scan_type: u8,
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub fn new(scan_type: Type, channel_mask: u32, duration: u8) -> Self {
        Self {
            scan_type: scan_type.into(),
            channel_mask,
            duration,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u32,
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
