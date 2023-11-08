pub const ID: u16 = 0x0021;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    have_current_network_key: bool,
    channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(have_current_network_key: bool, channel_mask: u32) -> Self {
        Self {
            have_current_network_key,
            channel_mask,
        }
    }

    #[must_use]
    pub const fn have_current_network_key(&self) -> bool {
        self.have_current_network_key
    }

    #[must_use]
    pub const fn channel_mask(&self) -> u32 {
        self.channel_mask
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
