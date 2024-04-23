use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::FromLeBytes;

const ID: u16 = 0x00A7;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    message: ByteSizedVec<u8>,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
