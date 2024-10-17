use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DA;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
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

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}
