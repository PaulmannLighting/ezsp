use crate::event;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;

const ID: u16 = 0x004E;

/// Gets information about a timer.
///
/// The Host can use this command to find out how much longer
/// it will be before a previously set timer will generate a callback.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    timer_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, timer_id: u8) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            timer_id,
        }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.timer_id])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [timer_id]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self { header, timer_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    time: u16,
    units: event::Units,
    repeat: bool,
}

impl Response {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        time: u16,
        units: event::Units,
        repeat: bool,
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Response {
    type Parameters = [u8; 4];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [time_low, time_high] = self.time.to_be_bytes();
        Some([time_low, time_high, self.units.into(), self.repeat.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [time @ .., units, repeat]: [u8; 4] = [0; 4];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            time: u16::from_be_bytes(time),
            units: event::Units::try_from(units)?,
            repeat: repeat != 0,
        })
    }
}
