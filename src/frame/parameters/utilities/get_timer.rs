use crate::ember::event::Units;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x004E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    timer_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(timer_id: u8) -> Self {
        Self { timer_id }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    time: u16,
    units: u8,
    repeat: bool,
}

impl Response {
    pub fn new(time: u16, units: Units, repeat: bool) -> Self {
        Self {
            time,
            units: units.into(),
            repeat,
        }
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
