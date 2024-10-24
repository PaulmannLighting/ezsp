use crate::ember::constants::COUNTER_TYPE_COUNT;
use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F1;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    values: [u16; COUNTER_TYPE_COUNT],
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for [u16; COUNTER_TYPE_COUNT] {
    fn from(response: Response) -> Self {
        response.values
    }
}
