
pub const ID: u16 = 0x008D;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    power: int8_t,
}

impl Response {
    #[must_use]
    pub const fn new(power: int8_t) -> Self {
        Self { power }
    }

    #[must_use]
    pub const fn power(&self) -> int8_t {
        self.power
    }
}
