use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00DA;

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
    zll_secondary_channel_mask: u32,
}

impl Response {
    #[must_use]
    pub const fn new(zll_secondary_channel_mask: u32) -> Self {
        Self {
            zll_secondary_channel_mask,
        }
    }

    #[must_use]
    pub const fn zll_secondary_channel_mask(&self) -> u32 {
        self.zll_secondary_channel_mask
    }
}
