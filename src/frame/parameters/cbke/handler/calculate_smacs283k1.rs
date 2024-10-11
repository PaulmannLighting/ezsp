use le_stream::derive::FromLeStream;

use crate::ember::{SmacData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x00EB;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    initiator_smac: SmacData,
    responder_smac: SmacData,
}

impl Handler {
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

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
