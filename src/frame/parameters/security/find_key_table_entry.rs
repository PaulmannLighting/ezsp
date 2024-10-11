use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0075;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    address: Eui64,
    link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(address: Eui64, link_key: bool) -> Self {
        Self { address, link_key }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    index: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = u8;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Ok(self.index)
    }
}
