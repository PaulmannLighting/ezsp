use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x001C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    channel: u8,
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(channel: u8, status: Status) -> Self {
        Self {
            channel,
            status: status.into(),
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
