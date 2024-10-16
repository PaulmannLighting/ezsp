use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D9;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    zll_primary_channel_mask: u32,
}

impl Response {
    #[must_use]
    pub const fn zll_primary_channel_mask(&self) -> u32 {
        self.zll_primary_channel_mask
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
