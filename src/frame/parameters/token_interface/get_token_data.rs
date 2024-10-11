use crate::ember::token::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0102;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    token: u32,
    index: u32,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32) -> Self {
        Self { token, index }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    token_data: Data,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Data;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.token_data)
    }
}
