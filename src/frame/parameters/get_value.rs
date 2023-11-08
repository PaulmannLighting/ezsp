
pub const ID: u16 = 0x00AA;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    value_id: EzspValueId,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: EzspValueId) -> Self {
        Self { value_id }
    }

    #[must_use]
    pub const fn value_id(&self) -> EzspValueId {
        self.value_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EzspStatus,
    value_length: u8,
    value: uint8_t[],
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, value_length: u8, value: uint8_t[]) -> Self {
        Self { status, value_length, value }
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
    pub const fn value(&self) -> uint8_t[] {
        self.value
    }
}
