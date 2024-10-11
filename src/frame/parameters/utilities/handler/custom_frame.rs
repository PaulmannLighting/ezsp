use crate::frame;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::FromLeStream;

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

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
