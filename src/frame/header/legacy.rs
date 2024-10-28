use le_stream::derive::{FromLeStream, ToLeStream};

use super::LowByte;

/// A legacy header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Legacy {
    sequence: u8,
    low_byte: LowByte,
    id: u8,
}

impl Legacy {
    #[must_use]
    pub fn new(sequence: u8, low_byte: LowByte, id: u8) -> Self {
        Self {
            sequence,
            low_byte,
            id,
        }
    }

    #[must_use]
    pub fn sequence(self) -> u8 {
        self.sequence
    }

    #[must_use]
    pub fn low_byte(self) -> LowByte {
        self.low_byte
    }

    #[must_use]
    pub fn id(self) -> u8 {
        self.id
    }
}
