use crate::event;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x000E;

/// Sets a timer on the NCP.
///
/// There are 2 independent timers available for use by the Host.
/// A timer can be cancelled by setting time to 0 or units to [`event::Units::Inactive`].
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    timer_id: u8,
    time: u16,
    units: event::Units,
    repeat: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        timer_id: u8,
        time: u16,
        units: event::Units,
        repeat: bool,
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            timer_id,
            time,
            units,
            repeat,
        }
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 5];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [time_low, time_high] = self.time.to_be_bytes();
        Some([
            self.timer_id,
            time_low,
            time_high,
            self.units.to_u8().expect("could not convert units to u8"),
            self.repeat.into(),
        ])
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.to_u8().expect("could not convert status to u8")])
    }
}
