use le_stream::derive::FromLeStream;

use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0054;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    payload: ByteSizedVec<u8>,
}

impl Handler {
    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
