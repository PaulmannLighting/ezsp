use crate::types::{EmberKeyStruct, EmberKeyType, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x006A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    key_type: EmberKeyType,
}

impl Command {
    #[must_use]
    pub const fn new(key_type: EmberKeyType) -> Self {
        Self { key_type }
    }

    #[must_use]
    pub const fn key_type(&self) -> EmberKeyType {
        self.key_type
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    key_struct: EmberKeyStruct,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, key_struct: EmberKeyStruct) -> Self {
        Self { status, key_struct }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn key_struct(&self) -> EmberKeyStruct {
        self.key_struct
    }
}
