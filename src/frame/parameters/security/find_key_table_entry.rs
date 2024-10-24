use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Identified;

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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    index: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for u8 {
    fn from(response: Response) -> Self {
        response.index
    }
}
