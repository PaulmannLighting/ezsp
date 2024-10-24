use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    zll_secondary_channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(zll_secondary_channel_mask: u32) -> Self {
        Self {
            zll_secondary_channel_mask,
        }
    }
}

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
