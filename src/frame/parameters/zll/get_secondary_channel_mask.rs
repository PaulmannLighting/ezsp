use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DA;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_secondary_channel_mask: u32,
}

impl Response {
    #[must_use]
    pub const fn zll_secondary_channel_mask(&self) -> u32 {
        self.zll_secondary_channel_mask
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
