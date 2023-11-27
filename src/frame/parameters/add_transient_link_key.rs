use crate::ember::key::Data;
use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00AF;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    partner: Eui64,
    transient_key: Data,
}

impl Command {
    #[must_use]
    pub const fn new(partner: Eui64, transient_key: Data) -> Self {
        Self {
            partner,
            transient_key,
        }
    }

    #[must_use]
    pub const fn partner(&self) -> Eui64 {
        self.partner
    }

    #[must_use]
    pub const fn transient_key(&self) -> &Data {
        &self.transient_key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
