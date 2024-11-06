//! Parameters for the [`Binding::binding_is_active`](crate::Binding::is_active) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x002E;

/// Command parameters
#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    active: bool,
}

impl Response {
    /// True if the binding table entry is active, false otherwise.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
