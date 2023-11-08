pub const ID: u16 = 0x0003;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    value_id: EzspExtendedValueId,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: EzspExtendedValueId, characteristics: u32) -> Self {
        Self {
            value_id,
            characteristics,
        }
    }

    #[must_use]
    pub const fn value_id(&self) -> EzspExtendedValueId {
        self.value_id
    }

    #[must_use]
    pub const fn characteristics(&self) -> u32 {
        self.characteristics
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
    value_length: u8,
    value: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, value_length: u8, value: ByteSizedVec<u8>) -> Self {
        Self {
            status,
            value_length,
            value,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
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
