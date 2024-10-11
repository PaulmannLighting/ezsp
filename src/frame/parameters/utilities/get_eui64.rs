use crate::ember::Eui64;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0026;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    #[must_use]
    pub fn eui64(self) -> Eui64 {
        self.eui64
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
