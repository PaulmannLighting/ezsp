use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D9;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    zll_primary_channel_mask: u32,
}

impl Response {
    #[must_use]
    pub const fn new(zll_primary_channel_mask: u32) -> Self {
        Self {
            zll_primary_channel_mask,
        }
    }

    #[must_use]
    pub const fn zll_primary_channel_mask(&self) -> u32 {
        self.zll_primary_channel_mask
    }
}
