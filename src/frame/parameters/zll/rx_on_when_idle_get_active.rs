//! Parameters for the [`Zll::rx_on_when_idle_get_active`](crate::Zll::rx_on_when_idle_get_active) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00D8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_rx_on_when_idle_get_active: bool,
}

impl Response {
    /// ZLL radio on when idle mode is active?
    #[must_use]
    pub const fn zll_rx_on_when_idle_get_active(&self) -> bool {
        self.zll_rx_on_when_idle_get_active
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
