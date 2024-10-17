use le_stream::derive::{FromLeStream, ToLeStream};

use super::{Header, HighByte, LowByte};

/// A legacy header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Legacy {
    sequence: u8,
    low_byte: LowByte,
    id: u8,
}

impl Header<u8> for Legacy {
    #[must_use]
    fn new(sequence: u8, low_byte: LowByte, id: u8) -> Self {
        Self {
            sequence,
            low_byte,
            id,
        }
    }

    #[must_use]
    fn sequence(self) -> u8 {
        self.sequence
    }

    #[must_use]
    fn low_byte(self) -> LowByte {
        self.low_byte
    }

    #[must_use]
    fn high_byte(self) -> Option<HighByte> {
        None
    }

    #[must_use]
    fn id(self) -> u8 {
        self.id
    }
}
