
pub const ID: u16 = 0x0009;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    token_id: u8,
    token_data: uint8_t[8],
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8, token_data: uint8_t[8]) -> Self {
        Self { token_id, token_data }
    }

    #[must_use]
    pub const fn token_id(&self) -> u8 {
        self.token_id
    }


    #[must_use]
    pub const fn token_data(&self) -> uint8_t[8] {
        self.token_data
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
