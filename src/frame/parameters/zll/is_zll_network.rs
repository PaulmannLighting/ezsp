//! Parameters for the [`Zll::is_zll_network`](crate::Zll::is_zll_network) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00BE;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    is_zll_network: bool,
}

impl Response {
    /// Returns whether the network is a ZLL network.
    #[must_use]
    pub const fn is_zll_network(&self) -> bool {
        self.is_zll_network
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
