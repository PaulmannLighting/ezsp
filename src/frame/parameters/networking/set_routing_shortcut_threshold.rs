use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00D0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    cost_thresh: u8,
}

impl Command {
    #[must_use]
    pub const fn new(cost_thresh: u8) -> Self {
        Self { cost_thresh }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}
impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
