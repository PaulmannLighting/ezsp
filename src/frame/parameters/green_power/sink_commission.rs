use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x010A;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    options: u8,
    gpm_addr_for_security: u16,
    gpm_addr_for_pairing: u16,
    sink_endpoint: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        options: u8,
        gpm_addr_for_security: u16,
        gpm_addr_for_pairing: u16,
        sink_endpoint: u8,
    ) -> Self {
        Self {
            options,
            gpm_addr_for_security,
            gpm_addr_for_pairing,
            sink_endpoint,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
