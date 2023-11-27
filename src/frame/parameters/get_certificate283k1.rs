use crate::ember::{Certificate283k1Data, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00EC;

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
    status: u8,
    local_cert: Certificate283k1Data,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, local_cert: Certificate283k1Data) -> Self {
        Self {
            status: status.into(),
            local_cert,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn local_cert(&self) -> &Certificate283k1Data {
        &self.local_cert
    }
}
