pub const ID: u16 = 0x0022;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(duration: u8) -> Self {
        Self { duration }
    }

    #[must_use]
    pub const fn duration(&self) -> u8 {
        self.duration
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
