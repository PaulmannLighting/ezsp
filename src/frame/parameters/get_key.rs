use crate::ember::key::{Struct, Type};
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x006A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    key_type: u8,
}

impl Command {
    #[must_use]
    pub fn new(key_type: Type) -> Self {
        Self {
            key_type: key_type.into(),
        }
    }

    pub fn key_type(&self) -> Result<Type, u8> {
        Type::try_from(self.key_type)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    key_struct: Struct,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, key_struct: Struct) -> Self {
        Self {
            status: status.into(),
            key_struct,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn key_struct(&self) -> &Struct {
        &self.key_struct
    }
}
