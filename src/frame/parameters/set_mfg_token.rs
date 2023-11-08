
pub const ID: u16 = 0x000C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    token_id: EzspMfgTokenId,
    token_data_length: u8,
    token_data: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(token_id: EzspMfgTokenId, token_data_length: u8, token_data: uint8_t[]) -> Self {
        Self { token_id, token_data_length, token_data }
    }

    #[must_use]
    pub const fn token_id(&self) -> EzspMfgTokenId {
        self.token_id
    }


    #[must_use]
    pub const fn token_data_length(&self) -> u8 {
        self.token_data_length
    }


    #[must_use]
    pub const fn token_data(&self) -> uint8_t[] {
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
