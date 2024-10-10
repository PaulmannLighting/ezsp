use le_stream::derive::ToLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x0006;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}
