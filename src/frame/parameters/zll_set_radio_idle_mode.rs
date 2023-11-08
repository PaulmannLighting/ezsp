
pub const ID: u16 = 0x00D4;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    mode: EmberRadioPowerMode,
}

impl Command {
    #[must_use]
    pub const fn new(mode: EmberRadioPowerMode) -> Self {
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> EmberRadioPowerMode {
        self.mode
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

