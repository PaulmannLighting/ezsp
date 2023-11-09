use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNetworkInitStruct,EmberStatus};

pub const ID: u16 = 0x0017;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    network_init_struct: EmberNetworkInitStruct,
}

impl Command {
    #[must_use]
    pub const fn new(network_init_struct: EmberNetworkInitStruct) -> Self {
        Self { network_init_struct }
    }

    #[must_use]
    pub const fn network_init_struct(&self) -> EmberNetworkInitStruct {
        self.network_init_struct
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
