use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_rx_on_when_idle_get_active: bool,
}

impl Response {
    #[must_use]
    pub const fn zll_rx_on_when_idle_get_active(&self) -> bool {
        self.zll_rx_on_when_idle_get_active
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
