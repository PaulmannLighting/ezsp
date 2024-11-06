//! Parameters for the [`Zll::operation_in_progress`](crate::Zll::operation_in_progress) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00D7;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    zll_operation_in_progress: bool,
}

impl Response {
    /// Returns whether a ZLL operation is in progress.
    #[must_use]
    pub const fn zll_operation_in_progress(&self) -> bool {
        self.zll_operation_in_progress
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
