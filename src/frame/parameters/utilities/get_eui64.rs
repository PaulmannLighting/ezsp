use crate::ember::Eui64;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0026;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    eui64: Eui64,
}

impl Response {
    #[must_use]
    pub const fn eui64(self) -> Eui64 {
        self.eui64
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for Eui64 {
    fn from(response: Response) -> Self {
        response.eui64
    }
}
