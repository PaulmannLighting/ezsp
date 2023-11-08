
pub const ID: u16 = 0x001C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    channel: u8,
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(channel: u8, status: EmberStatus) -> Self {
        Self { channel, status }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }


    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
