use le_stream::derive::{FromLeStream, ToLeStream};

use super::{Header, HighByte, LowByte};

/// An extended header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Extended {
    sequence: u8,
    low_byte: LowByte,
    high_byte: HighByte,
    id: u16,
}

impl Header<u8> for Extended {
    #[must_use]
    fn new(sequence: u8, low_byte: LowByte, id: u8) -> Self {
        Self {
            sequence,
            low_byte,
            high_byte: HighByte::default(),
            id: id as u16,
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
        Some(self.high_byte)
    }

    #[must_use]
    fn id(self) -> u8 {
        self.id as u8
    }
}

impl Header<u16> for Extended {
    #[must_use]
    fn new(sequence: u8, low_byte: LowByte, id: u16) -> Self {
        Self {
            sequence,
            low_byte,
            high_byte: HighByte::default(),
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
        Some(self.high_byte)
    }

    #[must_use]
    fn id(self) -> u16 {
        self.id
    }
}
