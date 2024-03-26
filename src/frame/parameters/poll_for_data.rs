use crate::ember::event::Units;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0042;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    interval: u16,
    units: u8,
    failure_limit: u8,
}

impl Command {
    #[must_use]
    pub fn new(interval: u16, units: Units, failure_limit: u8) -> Self {
        Self {
            interval,
            units: units.into(),
            failure_limit,
        }
    }

    #[must_use]
    pub const fn interval(&self) -> u16 {
        self.interval
    }

    pub fn units(&self) -> Result<Units, u8> {
        Units::try_from(self.units)
    }

    #[must_use]
    pub const fn failure_limit(&self) -> u8 {
        self.failure_limit
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
