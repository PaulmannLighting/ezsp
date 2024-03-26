use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00FB;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn phy_index(&self) -> u8 {
        self.phy_index
    }

    #[must_use]
    pub const fn page(&self) -> u8 {
        self.page
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
