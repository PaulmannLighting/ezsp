use super::payload::Payload;
use crate::frame::Parameter;
use crate::{frame, Resolve};
use le_stream::derive::{FromLeStream, ToLeStream};
use siliconlabs::Status;

const ID: u16 = 0x0112;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Payload;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
}
