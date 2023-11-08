
pub const ID: u16 = 0x00B5;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    duration_ms: u32,
}

impl Command {
    #[must_use]
    pub const fn new(duration_ms: u32) -> Self {
        Self { duration_ms }
    }

    #[must_use]
    pub const fn duration_ms(&self) -> u32 {
        self.duration_ms
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
