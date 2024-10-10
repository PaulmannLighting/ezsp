use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DB;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    zll_primary_channel_mask: u32,
}

impl Command {
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
