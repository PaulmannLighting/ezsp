
pub const ID: u16 = 0x0105;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    config: u8,
    min_acks_needed: u8,
}

impl Command {
    #[must_use]
    pub const fn new(config: u8, min_acks_needed: u8) -> Self {
        Self { config, min_acks_needed }
    }

    #[must_use]
    pub const fn config(&self) -> u8 {
        self.config
    }


    #[must_use]
    pub const fn min_acks_needed(&self) -> u8 {
        self.min_acks_needed
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
