use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x0078;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
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
