//! Parameters for the [`Zll::get_primary_channel_mask`](crate::Zll::get_primary_channel_mask) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00D9;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_primary_channel_mask: u32,
}

impl Response {
    /// ZLL primary channel mask.
    #[must_use]
    pub const fn zll_primary_channel_mask(&self) -> u32 {
        self.zll_primary_channel_mask
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
