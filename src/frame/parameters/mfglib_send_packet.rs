
pub const ID: u16 = 0x0089;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    packet_length: u8,
    packet_contents: uint8_t[],
}

impl Command {
    #[must_use]
    pub const fn new(packet_length: u8, packet_contents: uint8_t[]) -> Self {
        Self { packet_length, packet_contents }
    }

    #[must_use]
    pub const fn packet_length(&self) -> u8 {
        self.packet_length
    }


    #[must_use]
    pub const fn packet_contents(&self) -> uint8_t[] {
        self.packet_contents
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
