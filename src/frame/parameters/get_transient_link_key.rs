use crate::ember::key::TransientData;
use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00CE;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    eui: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: Eui64) -> Self {
        Self { eui }
    }

    #[must_use]
    pub const fn eui(&self) -> Eui64 {
        self.eui
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    transient_key_data: TransientData,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, transient_key_data: TransientData) -> Self {
        Self {
            status: status.into(),
            transient_key_data,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn transient_key_data(&self) -> &TransientData {
        &self.transient_key_data
    }
}
