pub const ID: u16 = 0x0015;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(code: u16) -> Self {
        Self { code }
    }

    #[must_use]
    pub const fn code(&self) -> u16 {
        self.code
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;
