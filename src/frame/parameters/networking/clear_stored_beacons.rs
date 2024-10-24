use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x003C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
