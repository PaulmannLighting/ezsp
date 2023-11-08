pub const ID: u16 = 0x00A0;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
