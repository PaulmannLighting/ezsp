pub const ID: u16 = 0x0089;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    packet_length: u8,
    packet_contents: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(packet_length: u8, packet_contents: ByteSizedVec<u8>) -> Self {
        Self {
            packet_length,
            packet_contents,
        }
    }

    #[must_use]
    pub const fn packet_length(&self) -> u8 {
        self.packet_length
    }

    #[must_use]
    pub const fn packet_contents(&self) -> ByteSizedVec<u8> {
        self.packet_contents
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
