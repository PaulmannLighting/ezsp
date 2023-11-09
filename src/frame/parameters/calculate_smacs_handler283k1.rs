use crate::types::{EmberSmacData, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00EB;

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
    initiator_smac: EmberSmacData,
    responder_smac: EmberSmacData,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: EmberStatus,
        initiator_smac: EmberSmacData,
        responder_smac: EmberSmacData,
    ) -> Self {
        Self {
            status,
            initiator_smac,
            responder_smac,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn initiator_smac(&self) -> EmberSmacData {
        self.initiator_smac
    }

    #[must_use]
    pub const fn responder_smac(&self) -> EmberSmacData {
        self.responder_smac
    }
}
