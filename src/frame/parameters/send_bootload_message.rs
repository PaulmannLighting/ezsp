
pub const ID: u16 = 0x0090;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    broadcast: bool,
    dest_eui64: EmberEUI64,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: EmberEUI64, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { broadcast, dest_eui64, message_length, message_contents }
    }

    #[must_use]
    pub const fn broadcast(&self) -> bool {
        self.broadcast
    }


    #[must_use]
    pub const fn dest_eui64(&self) -> EmberEUI64 {
        self.dest_eui64
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
