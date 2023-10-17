use crate::ember_status::EmberStatus;
use crate::event;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;

pub const ID: u16 = 0x000E;

/// Sets a timer on the NCP.
///
/// There are 2 independent timers available for use by the Host.
/// A timer can be cancelled by setting time to 0 or units to [`event::Units::Inactive`].
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    timer_id: u8,
    time: u16,
    units: event::Units,
    repeat: bool,
}

impl Command {
    #[must_use]
    pub const fn new(timer_id: u8, time: u16, units: event::Units, repeat: bool) -> Self {
        Self {
            timer_id,
            time,
            units,
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

    #[must_use]
    pub const fn units(&self) -> event::Units {
        self.units
    }

    #[must_use]
    pub const fn repeat(&self) -> bool {
        self.repeat
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        let [time_low, time_high] = self.time.to_be_bytes();
        [
            self.timer_id,
            time_low,
            time_high,
            self.units.into(),
            self.repeat.into(),
        ]
        .into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [timer_id, time @ .., units, repeat] = [0; 5];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            timer_id,
            time: u16::from_be_bytes(time),
            units: event::Units::try_from(units)?,
            repeat: repeat != 0,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: EmberStatus::try_from(status)?,
        })
    }
}
