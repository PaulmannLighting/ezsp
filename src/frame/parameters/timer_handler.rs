
pub const ID: u16 = 0x000F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    timer_id: u8,
}

impl Response {
    #[must_use]
    pub const fn new(timer_id: u8) -> Self {
        Self { timer_id }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}
