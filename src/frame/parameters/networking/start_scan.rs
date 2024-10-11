use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::Status;

use crate::ezsp::network::scan::Type;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x001A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
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
