pub const ID: u16 = 0x00D6;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    state: u16,
}

impl Command {
    #[must_use]
    pub const fn new(state: u16) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> u16 {
        self.state
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;
