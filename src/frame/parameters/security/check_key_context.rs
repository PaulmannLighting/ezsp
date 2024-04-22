use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;
use siliconlabs::zigbee::security::ManContext;
use siliconlabs::Status;

const ID: u16 = 0x0110;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    context: ManContext,
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u32,
}

impl Response {
    pub fn status(&self) -> Result<Status, u32> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
