use le_stream::derive::FromLeBytes;

use crate::ember::Status;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0093;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn message(self) -> ByteSizedVec<u8> {
        self.message
    }
}
