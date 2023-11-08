pub const ID: u16 = 0x0047;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    payload_length: u8,
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(payload_length: u8, payload: ByteSizedVec<u8>) -> Self {
        Self {
            payload_length,
            payload,
        }
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload_length
    }

    #[must_use]
    pub const fn payload(&self) -> ByteSizedVec<u8> {
        self.payload
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    reply_length: u8,
    reply: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, reply_length: u8, reply: ByteSizedVec<u8>) -> Self {
        Self {
            status,
            reply_length,
            reply,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn reply_length(&self) -> u8 {
        self.reply_length
    }

    #[must_use]
    pub const fn reply(&self) -> ByteSizedVec<u8> {
        self.reply
    }
}
