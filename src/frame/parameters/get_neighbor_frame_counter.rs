pub const ID: u16 = 0x003E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: EmberEUI64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    return_frame_counter: u32,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, return_frame_counter: u32) -> Self {
        Self {
            status,
            return_frame_counter,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn return_frame_counter(&self) -> u32 {
        self.return_frame_counter
    }
}
