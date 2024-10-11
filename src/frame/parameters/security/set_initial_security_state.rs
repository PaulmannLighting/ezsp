use crate::ember::security::initial::State;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0068;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    state: State,
}

impl Command {
    #[must_use]
    pub const fn new(state: State) -> Self {
        Self { state }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    success: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.success).resolve()
    }
}
