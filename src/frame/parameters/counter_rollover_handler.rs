use crate::ember::counter::Type;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00F2;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    typ: u8,
}

impl Response {
    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }
}
