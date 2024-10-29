use le_stream::derive::{FromLeStream, ToLeStream};

use super::{HighByte, LowByte};

/// An extended header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Extended {
    sequence: u8,
    low_byte: LowByte,
    high_byte: HighByte,
    id: u16,
}

impl Extended {
    #[must_use]
    pub const fn new(sequence: u8, low_byte: LowByte, id: u16) -> Self {
        Self {
            sequence,
            low_byte,
            high_byte: HighByte::FRAME_FORMAT_VERSION_0,
            id,
        }
    }

    #[must_use]
    pub const fn sequence(self) -> u8 {
        self.sequence
    }

    #[must_use]
    pub const fn low_byte(self) -> LowByte {
        self.low_byte
    }

    #[must_use]
    pub const fn high_byte(self) -> HighByte {
        self.high_byte
    }

    #[must_use]
    pub const fn id(self) -> u16 {
        self.id
    }
}
