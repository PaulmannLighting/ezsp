use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x00C7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    gpep_handle: u8,
}

impl Handler {
    pub fn status(self) -> Result<u8, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.gpep_handle)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
