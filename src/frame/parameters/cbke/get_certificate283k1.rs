use crate::ember::{Certificate283k1Data, Status};
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00EC;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    local_cert: Certificate283k1Data,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn local_cert(&self) -> &Certificate283k1Data {
        &self.local_cert
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
