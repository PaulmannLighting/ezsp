use crate::ember::counter::Type;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00F2;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    typ: u8,
}

impl Response {
    #[must_use]
    pub fn new(typ: Type) -> Self {
        Self { typ: typ.into() }
    }

    pub fn typ(&self) -> Result<Type, u8> {
        Type::try_from(self.typ)
    }
}
