use crate::ember::event::Units;
use crate::ember::Status;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x000E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    timer_id: u8,
    time: u16,
    units: u8,
    repeat: bool,
}

impl Command {
    #[must_use]
    pub fn new(timer_id: u8, time: u16, units: Units, repeat: bool) -> Self {
        Self {
            timer_id,
            time,
            units: units.into(),
            repeat,
        }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }

    #[must_use]
    pub const fn time(&self) -> u16 {
        self.time
    }

    pub fn units(&self) -> Result<Units, u8> {
        Units::try_from(self.units)
    }

    #[must_use]
    pub const fn repeat(&self) -> bool {
        self.repeat
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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
