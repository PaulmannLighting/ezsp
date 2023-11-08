
pub const ID: u16 = 0x00D0;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    cost_thresh: u8,
}

impl Command {
    #[must_use]
    pub const fn new(cost_thresh: u8) -> Self {
        Self { cost_thresh }
    }

    #[must_use]
    pub const fn cost_thresh(&self) -> u8 {
        self.cost_thresh
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
