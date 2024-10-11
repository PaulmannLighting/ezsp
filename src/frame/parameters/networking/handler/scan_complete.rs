use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x001C;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    channel: u8,
    status: u8,
}

impl Handler {
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
