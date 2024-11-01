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
    /// Creates a new legacy header.
    #[must_use]
    pub const fn new(sequence: u8, low_byte: LowByte, id: u8) -> Self {
        Self {
            sequence,
            low_byte,
            id,
        }
    }

    /// Returns the sequence number.
    #[must_use]
    pub const fn sequence(self) -> u8 {
        self.sequence
    }

    /// Returns the low byte.
    #[must_use]
    pub const fn low_byte(self) -> LowByte {
        self.low_byte
    }

    /// Returns the ID.
    #[must_use]
    pub const fn id(self) -> u8 {
        self.id
    }
}
