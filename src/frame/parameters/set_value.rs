pub const ID: u16 = 0x00AB;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    value_id: EzspValueId,
    value_length: u8,
    value: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: EzspValueId, value_length: u8, value: ByteSizedVec<u8>) -> Self {
        Self {
            value_id,
            value_length,
            value,
        }
    }

    #[must_use]
    pub const fn value_id(&self) -> EzspValueId {
        self.value_id
    }

    #[must_use]
    pub const fn value_length(&self) -> u8 {
        self.value_length
    }

    #[must_use]
    pub const fn value(&self) -> ByteSizedVec<u8> {
        self.value
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }
}
