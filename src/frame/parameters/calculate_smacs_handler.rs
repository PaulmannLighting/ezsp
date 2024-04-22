use le_stream::derive::FromLeBytes;

use crate::ember::{SmacData, Status};

const ID: u16 = 0x00A0;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    initiator_smac: SmacData,
    responder_smac: SmacData,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, initiator_smac: SmacData, responder_smac: SmacData) -> Self {
        Self {
            status: status.into(),
            initiator_smac,
            responder_smac,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn initiator_smac(&self) -> &SmacData {
        &self.initiator_smac
    }

    #[must_use]
    pub const fn responder_smac(&self) -> &SmacData {
        &self.responder_smac
    }
}
