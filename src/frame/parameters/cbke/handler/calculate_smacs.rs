use le_stream::derive::FromLeStream;

use crate::ember::{SmacData, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x00A0;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    payload: Payload,
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Handler {
    type Output = Payload;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
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
