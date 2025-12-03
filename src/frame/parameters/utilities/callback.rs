//! Parameters for the [`Utilities::callback`](crate::Utilities::callback) command.

use le_stream::ToLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0006;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}
