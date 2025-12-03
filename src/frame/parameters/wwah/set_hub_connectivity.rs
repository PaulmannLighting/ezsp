//! Parameters for the [`Wwah::set_hub_connectivity`](crate::Wwah::set_hub_connectivity) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00E4;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    connected: bool,
}

impl Command {
    #[must_use]
    pub const fn new(connected: bool) -> Self {
        Self { connected }
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
