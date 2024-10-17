use crate::ember::constants::COUNTER_TYPE_COUNT;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0065;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    values: [u16; COUNTER_TYPE_COUNT],
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl From<Response> for [u16; COUNTER_TYPE_COUNT] {
    fn from(response: Response) -> Self {
        response.values
    }
}
