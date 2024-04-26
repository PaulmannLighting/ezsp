use le_stream::derive::FromLeBytes;

use crate::frame::Parameter;

const ID: u16 = 0x006E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    sequence_number: u8,
}

impl Handler {
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
