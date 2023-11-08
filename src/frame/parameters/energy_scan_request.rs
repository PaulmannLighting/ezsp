
pub const ID: u16 = 0x009C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    target: EmberNodeId,
    scan_channels: u32,
    scan_duration: u8,
    scan_count: u16,
}

impl Command {
    #[must_use]
    pub const fn new(target: EmberNodeId, scan_channels: u32, scan_duration: u8, scan_count: u16) -> Self {
        Self { target, scan_channels, scan_duration, scan_count }
    }

    #[must_use]
    pub const fn target(&self) -> EmberNodeId {
        self.target
    }


    #[must_use]
    pub const fn scan_channels(&self) -> u32 {
        self.scan_channels
    }


    #[must_use]
    pub const fn scan_duration(&self) -> u8 {
        self.scan_duration
    }


    #[must_use]
    pub const fn scan_count(&self) -> u16 {
        self.scan_count
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
