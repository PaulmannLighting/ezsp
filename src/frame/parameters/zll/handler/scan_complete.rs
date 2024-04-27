use le_stream::derive::FromLeBytes;

use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x00B7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    status: u8,
}

impl Handler {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
