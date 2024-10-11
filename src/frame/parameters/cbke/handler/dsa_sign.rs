use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x00A7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
