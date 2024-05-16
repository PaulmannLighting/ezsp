use crate::ember::multi_phy::nwk::Config;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00F8;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    phy_index: u8,
    page: u8,
    channel: u8,
    power: i8,
    bitmask: u8,
}

impl Command {
    #[must_use]
    pub fn new(phy_index: u8, page: u8, channel: u8, power: i8, bitmask: Config) -> Self {
        Self {
            phy_index,
            page,
            channel,
            power,
            bitmask: bitmask.into(),
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

    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
    }

    #[must_use]
    pub fn bitmask(&self) -> Option<Config> {
        if self.bitmask & Into::<u8>::into(Config::BroadcastSupport) != 0 {
            Some(Config::BroadcastSupport)
        } else {
            None
        }
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
