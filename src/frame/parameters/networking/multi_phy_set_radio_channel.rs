use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x00FB;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    phy_index: u8,
    page: u8,
    channel: u8,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8, page: u8, channel: u8) -> Self {
        Self {
            phy_index,
            page,
            channel,
        }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
