
pub const ID: u16 = 0x0103;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    token: u32,
    index: u32,
    token_data: EmberTokenData,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32, token_data: EmberTokenData) -> Self {
        Self { token, index, token_data }
    }

    #[must_use]
    pub const fn token(&self) -> u32 {
        self.token
    }


    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }


    #[must_use]
    pub const fn token_data(&self) -> EmberTokenData {
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
