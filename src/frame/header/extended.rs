use core::fmt::{self, Display};

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
    /// Creates a new extended header.
    #[must_use]
    pub const fn new(sequence: u8, low_byte: LowByte, id: u16) -> Self {
        Self {
            sequence,
            low_byte,
            high_byte: HighByte::FRAME_FORMAT_VERSION_0,
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

    /// Returns the high byte.
    #[must_use]
    pub const fn high_byte(self) -> HighByte {
        self.high_byte
    }

    /// Returns the ID.
    #[must_use]
    pub const fn id(self) -> u16 {
        self.id
    }
}

impl Display for Extended {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Extended {{ sequence: {:#04X}, low_byte: {}, high_byte: {}, id: {:#06X}}}",
            self.sequence, self.low_byte, self.high_byte, self.id
        )
    }
}
