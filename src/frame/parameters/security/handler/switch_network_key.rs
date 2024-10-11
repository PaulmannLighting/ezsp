use le_stream::derive::FromLeStream;

use crate::frame::Parameter;

const ID: u16 = 0x006E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    sequence_number: u8,
}

impl Handler {
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Handler {
    const ID: u16 = ID;
}
