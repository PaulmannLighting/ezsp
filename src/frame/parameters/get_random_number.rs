
pub const ID: u16 = 0x0049;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, value: u16) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}
