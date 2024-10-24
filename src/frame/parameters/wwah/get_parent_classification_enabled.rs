use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    enabled: bool,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.enabled
    }
}
