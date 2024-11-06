//! Parameters for the [`Zll::set_primary_channel_mask`](crate::Zll::set_primary_channel_mask) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00DB;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    zll_primary_channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(zll_primary_channel_mask: u32) -> Self {
        Self {
            zll_primary_channel_mask,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
