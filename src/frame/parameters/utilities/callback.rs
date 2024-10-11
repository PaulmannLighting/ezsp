use le_stream::derive::ToLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0006;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}
