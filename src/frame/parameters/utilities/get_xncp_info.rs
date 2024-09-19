use crate::ember::Status;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0013;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: u8,
    manufacturer_id: u16,
    version_number: u16,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, manufacturer_id: u16, version_number: u16) -> Self {
        Self {
            status: status.into(),
            manufacturer_id,
            version_number,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn manufacturer_id(&self) -> u16 {
        self.manufacturer_id
    }

    #[must_use]
    pub const fn version_number(&self) -> u16 {
        self.version_number
    }
}
