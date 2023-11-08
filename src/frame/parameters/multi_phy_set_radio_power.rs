
pub const ID: u16 = 0x00FA;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    phy_index: u8,
    power: int8_t,
}

impl Command {
    #[must_use]
    pub const fn new(phy_index: u8, power: int8_t) -> Self {
        Self { phy_index, power }
    }

    #[must_use]
    pub const fn phy_index(&self) -> u8 {
        self.phy_index
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
