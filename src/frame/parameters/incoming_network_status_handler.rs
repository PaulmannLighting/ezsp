
pub const ID: u16 = 0x00C4;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    error_code: u8,
    target: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(error_code: u8, target: EmberNodeId) -> Self {
        Self { error_code, target }
    }

    #[must_use]
    pub const fn error_code(&self) -> u8 {
        self.error_code
    }


    #[must_use]
    pub const fn target(&self) -> EmberNodeId {
        self.target
    }
}
