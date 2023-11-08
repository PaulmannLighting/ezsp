
pub const ID: u16 = 0x00E0;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    sink_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8) -> Self {
        Self { sink_index }
    }

    #[must_use]
    pub const fn sink_index(&self) -> u8 {
        self.sink_index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

