use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    enabled: bool,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.enabled
    }
}
