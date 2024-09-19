use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00FB;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
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

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
