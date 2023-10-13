use crate::event;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use num_traits::ToPrimitive;

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
    pub const fn new(sequence: u8, control: Control, timer_id: u8) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            timer_id,
        }
    }

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
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    time: u16,
    units: event::Units,
    repeat: bool,
}

impl Response {
    pub const fn new(
        sequence: u8,
        control: Control,
        time: u16,
        units: event::Units,
        repeat: bool,
    ) -> Self {
        Self {
            header: Self::make_header(sequence, control),
            time,
            units,
            repeat,
        }
    }

    pub const fn time(&self) -> u16 {
        self.time
    }

    pub const fn units(&self) -> &event::Units {
        &self.units
    }

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
        Some([
            time_low,
            time_high,
            self.units.to_u8().expect("could not convert units to u8"),
            self.repeat.into(),
        ])
    }
}
