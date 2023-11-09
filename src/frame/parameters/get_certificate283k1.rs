use crate::types::{EmberCertificate283k1Data, EmberStatus};
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
    status: EmberStatus,
    local_cert: EmberCertificate283k1Data,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, local_cert: EmberCertificate283k1Data) -> Self {
        Self { status, local_cert }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn local_cert(&self) -> EmberCertificate283k1Data {
        self.local_cert
    }
}
