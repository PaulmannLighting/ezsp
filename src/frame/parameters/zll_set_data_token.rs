
pub const ID: u16 = 0x00BD;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    data: EmberTokTypeStackZllData,
}

impl Command {
    #[must_use]
    pub const fn new(data: EmberTokTypeStackZllData) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> EmberTokTypeStackZllData {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

