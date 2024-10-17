//! Gets information about a timer.
//!
//! The Host can use this command to find out how much longer it will be
//! before a previously set timer will generate a callback.

use crate::ember::event::{Duration, Units};
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x004E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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
    const ID: Option<Self::Id> = Some(ID);
}

/// The response to a get timer command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    time: u16,
    units: u8,
    repeat: bool,
}

impl Response {
    /// Returns the time.
    #[must_use]
    pub const fn time(&self) -> u16 {
        self.time
    }

    /// Returns the time units.
    ///
    /// # Errors
    ///
    /// Returns an error if the units are invalid.
    pub fn units(&self) -> Result<Units, u8> {
        Units::try_from(self.units)
    }

    /// Returns the time duration.
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.units()
            .ok()
            .and_then(|units| Duration::try_new(self.time, units).ok())
    }

    /// Returns whether the timer should repeat.
    #[must_use]
    pub const fn repeat(&self) -> bool {
        self.repeat
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}
