use le_stream::derive::FromLeBytes;

use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0054;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    payload: ByteSizedVec<u8>,
}

impl Handler {
    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
