use le_stream::derive::FromLeStream;

use crate::ember::{SmacData, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x00EB;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    payload: Payload,
}

impl Handler {
    pub fn result(self) -> Result<Payload, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.payload)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Payload {
    initiator_smac: SmacData,
    responder_smac: SmacData,
}

impl Payload {
    #[must_use]
    pub const fn initiator_smac(&self) -> &SmacData {
        &self.initiator_smac
    }

    #[must_use]
    pub const fn responder_smac(&self) -> &SmacData {
        &self.responder_smac
    }
}
