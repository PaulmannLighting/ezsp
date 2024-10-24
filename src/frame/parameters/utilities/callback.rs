use le_stream::derive::ToLeStream;

use crate::frame::Identified;

const ID: u16 = 0x0006;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}
