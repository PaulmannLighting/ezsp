//! Parameters for the [`TokenInterface::token_factory_reset`](crate::TokenInterface::token_factory_reset) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0077;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    exclude_outgoing_fc: bool,
    exclude_boot_counter: bool,
}

impl Parameter for Command {
    const ID: u16 = ID;
}

impl Command {
    #[must_use]
    pub const fn new(exclude_outgoing_fc: bool, exclude_boot_counter: bool) -> Self {
        Self {
            exclude_outgoing_fc,
            exclude_boot_counter,
        }
    }
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
