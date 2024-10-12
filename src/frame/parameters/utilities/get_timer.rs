use crate::ember::event::{Duration, Units};
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x004E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    timer_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(timer_id: u8) -> Self {
        Self { timer_id }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
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
    pub fn duration(&self) -> Option<Duration> {
        self.units()
            .ok()
            .and_then(|units| Duration::try_new(self.time, units).ok())
    }

    #[must_use]
    pub const fn repeat(&self) -> bool {
        self.repeat
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
