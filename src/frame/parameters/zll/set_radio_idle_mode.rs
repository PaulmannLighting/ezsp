//! Parameters for the [`Zll::set_radio_idle_mode`](crate::Zll::set_radio_idle_mode) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::radio::PowerMode;
use crate::frame::Parameter;

const ID: u16 = 0x00D4;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub fn new(mode: PowerMode) -> Self {
        Self { mode: mode.into() }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
