use le_stream::derive::ToLeBytes;

use crate::frame::Parameter;

const ID: u16 = 0x0006;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}
