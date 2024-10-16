use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x0019;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
}

impl Handler {
    pub fn result(&self) -> Result<(), Error> {
        Status::try_from(self.status).resolve()
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
