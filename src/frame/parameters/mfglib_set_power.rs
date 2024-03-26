use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x008C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    tx_power_mode: u16,
    power: i8,
}

impl Command {
    #[must_use]
    pub const fn new(tx_power_mode: u16, power: i8) -> Self {
        Self {
            tx_power_mode,
            power,
        }
    }

    #[must_use]
    pub const fn tx_power_mode(&self) -> u16 {
        self.tx_power_mode
    }

    #[must_use]
    pub const fn power(&self) -> i8 {
        self.power
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
