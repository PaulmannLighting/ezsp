
pub const ID: u16 = 0x00E7;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    enabled: bool,
}

impl Command {
    #[must_use]
    pub const fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

