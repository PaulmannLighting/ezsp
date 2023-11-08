
pub const ID: u16 = 0x00A7;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { status, message_length, message_contents }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
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
