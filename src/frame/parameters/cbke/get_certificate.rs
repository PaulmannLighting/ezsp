use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{CertificateData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x00A5;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    local_cert: CertificateData,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn local_cert(&self) -> &CertificateData {
        &self.local_cert
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
