
pub const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    payload_length: u8,
    payload: uint8_t[],
}

impl Response {
    #[must_use]
    pub const fn new(payload_length: u8, payload: uint8_t[]) -> Self {
        Self { payload_length, payload }
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload_length
    }


    #[must_use]
    pub const fn payload(&self) -> uint8_t[] {
        self.payload
    }
}
