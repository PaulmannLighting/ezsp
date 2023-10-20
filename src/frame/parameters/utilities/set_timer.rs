use crate::ember::event;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

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
    type IntoIter = Chain<
        Chain<Chain<Once<Self::Item>, IntoIter<Self::Item, 2>>, Once<Self::Item>>,
        Once<Self::Item>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.timer_id)
            .chain(self.time.to_be_bytes())
            .chain(once(self.units.into()))
            .chain(once(self.repeat.into()))
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let timer_id = src.read_num_be()?;
        let time = src.read_num_be()?;
        let units: u8 = src.read_num_be()?;
        let repeat = src.read_bool()?;
        Ok(Self {
            timer_id,
            time,
            units: units.try_into()?,
            repeat,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
