//! Custom frame handler.

use le_stream::{FromLeStream, Prefixed};

use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0054;

/// A callback indicating a custom `EZSP` message has been received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    payload: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Handler {
    /// The payload of the custom frame.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        self.payload.as_ref()
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
