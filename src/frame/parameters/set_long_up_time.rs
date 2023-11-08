pub const ID: u16 = 0x00E3;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    has_long_up_time: bool,
}

impl Command {
    #[must_use]
    pub const fn new(has_long_up_time: bool) -> Self {
        Self { has_long_up_time }
    }

    #[must_use]
    pub const fn has_long_up_time(&self) -> bool {
        self.has_long_up_time
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;
