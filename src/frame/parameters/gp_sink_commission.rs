use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x010A;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    options: u8,
    gpm_addr_for_security: u16,
    gpm_addr_for_pairing: u16,
    sink_endpoint: u8,
}

impl Command {
    #[must_use]
    pub const fn new(options: u8, gpm_addr_for_security: u16, gpm_addr_for_pairing: u16, sink_endpoint: u8) -> Self {
        Self { options, gpm_addr_for_security, gpm_addr_for_pairing, sink_endpoint }
    }

    #[must_use]
    pub const fn options(&self) -> u8 {
        self.options
    }


    #[must_use]
    pub const fn gpm_addr_for_security(&self) -> u16 {
        self.gpm_addr_for_security
    }


    #[must_use]
    pub const fn gpm_addr_for_pairing(&self) -> u16 {
        self.gpm_addr_for_pairing
    }


    #[must_use]
    pub const fn sink_endpoint(&self) -> u8 {
        self.sink_endpoint
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
