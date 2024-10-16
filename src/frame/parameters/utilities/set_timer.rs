use crate::ember::event::Units;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x000E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve()
    }
}
