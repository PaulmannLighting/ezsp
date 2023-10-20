use crate::event;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x004E;

/// Gets information about a timer.
///
/// The Host can use this command to find out how much longer
/// it will be before a previously set timer will generate a callback.
#[derive(Debug, Eq, PartialEq)]
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        self.timer_id.to_be_bytes().into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            timer_id: src.read_num_be()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    time: u16,
    units: event::Units,
    repeat: bool,
}

impl Response {
    #[must_use]
    pub const fn new(time: u16, units: event::Units, repeat: bool) -> Self {
        Self {
            time,
            units,
            repeat,
        }
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Chain<Chain<IntoIter<Self::Item, 2>, Once<Self::Item>>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.time
            .to_be_bytes()
            .into_iter()
            .chain(once(self.units.into()))
            .chain(once(self.repeat.into()))
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let time = src.read_num_be()?;
        let units: u8 = src.read_num_be()?;
        let repeat = src.read_bool()?;
        Ok(Self {
            time,
            units: units.try_into()?,
            repeat,
        })
    }
}
