pub const ID: u16 = 0x0095;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    channel: u8,
}

impl Command {
    #[must_use]
    pub const fn new(channel: u8) -> Self {
        Self { channel }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
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
