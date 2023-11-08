pub const ID: u16 = 0x00DC;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    zll_secondary_channel_mask: u32,
}

impl Command {
    #[must_use]
    pub const fn new(zll_secondary_channel_mask: u32) -> Self {
        Self {
            zll_secondary_channel_mask,
        }
    }

    #[must_use]
    pub const fn zll_secondary_channel_mask(&self) -> u32 {
        self.zll_secondary_channel_mask
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;
