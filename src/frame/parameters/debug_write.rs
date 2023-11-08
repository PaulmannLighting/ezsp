
pub const ID: u16 = 0x0012;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    binary_message: bool,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(binary_message: bool, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { binary_message, message_length, message_contents }
    }

    #[must_use]
    pub const fn binary_message(&self) -> bool {
        self.binary_message
    }


    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }


    #[must_use]
    pub const fn message_contents(&self) -> uint8_t[] {
        self.message_contents
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
