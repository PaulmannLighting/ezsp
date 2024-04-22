use crate::ember::Status;
use crate::types::ByteSizedVec;
use le_stream::derive::FromLeBytes;

const ID: u16 = 0x00A7;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, message: ByteSizedVec<u8>) -> Self {
        Self {
            status: status.into(),
            message,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}
