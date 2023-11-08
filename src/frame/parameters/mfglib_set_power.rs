
pub const ID: u16 = 0x008C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    tx_power_mode: u16,
    power: int8_t,
}

impl Command {
    #[must_use]
    pub const fn new(tx_power_mode: u16, power: int8_t) -> Self {
        Self { tx_power_mode, power }
    }

    #[must_use]
    pub const fn tx_power_mode(&self) -> u16 {
        self.tx_power_mode
    }


    #[must_use]
    pub const fn power(&self) -> int8_t {
        self.power
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
