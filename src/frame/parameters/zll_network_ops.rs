pub const ID: u16 = 0x00B2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    network_info: EmberZllNetwork,
    op: EzspZllNetworkOperation,
    radio_tx_power: int8_t,
}

impl Command {
    #[must_use]
    pub const fn new(
        network_info: EmberZllNetwork,
        op: EzspZllNetworkOperation,
        radio_tx_power: int8_t,
    ) -> Self {
        Self {
            network_info,
            op,
            radio_tx_power,
        }
    }

    #[must_use]
    pub const fn network_info(&self) -> EmberZllNetwork {
        self.network_info
    }

    #[must_use]
    pub const fn op(&self) -> EzspZllNetworkOperation {
        self.op
    }

    #[must_use]
    pub const fn radio_tx_power(&self) -> int8_t {
        self.radio_tx_power
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
