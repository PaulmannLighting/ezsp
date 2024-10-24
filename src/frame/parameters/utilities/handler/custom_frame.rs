//! Custom frame handler.

use le_stream::derive::FromLeStream;

use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0054;

/// A callback indicating a custom EZSP message has been received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    payload: ByteSizedVec<u8>,
}

impl Handler {
    /// The payload of the custom frame.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
