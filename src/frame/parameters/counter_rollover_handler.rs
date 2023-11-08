
pub const ID: u16 = 0x00F2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    type: EmberCounterType,
}

impl Response {
    #[must_use]
    pub const fn new(type: EmberCounterType) -> Self {
        Self { type }
    }

    #[must_use]
    pub const fn type(&self) -> EmberCounterType {
        self.type
    }
}
