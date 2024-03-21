pub use callback_type::CallbackType;
use le_stream::{FromLeBytes, ToLeBytes};
use log::warn;
use num_traits::FromPrimitive;
pub use sleep_mode::SleepMode;
use std::iter::{once, Once};

mod callback_type;
mod sleep_mode;

const COMMAND_MASK: u8 = 0b1000_0000;
const OVERFLOW_MASK: u8 = 0b0000_0001;
const TRUNCATED_MASK: u8 = 0b0000_0010;
const CALLBACK_PENDING_MASK: u8 = 0b0000_0100;
const CALLBACK_TYPE_MASK_HIGH: u8 = 0b0001_0000;
const CALLBACK_TYPE_MASK_LOW: u8 = 0b0000_1000;
const CALLBACK_OFFSET_HIGH: u8 = 3;
const CALLBACK_OFFSET_LOW: u8 = 2;
const NETWORK_INDEX_MASK_HIGH: u8 = 0b0100_0000;
const NETWORK_INDEX_MASK_LOW: u8 = 0b0010_0000;
const NETWORK_INDEX_MASK: u8 = NETWORK_INDEX_MASK_HIGH + NETWORK_INDEX_MASK_LOW;
const NETWORK_INDEX_OFFSET_HIGH: u8 = 6;
const NETWORK_INDEX_OFFSET_LOW: u8 = 4;
const SLEEP_MODE_MASK_LOW: u8 = 0b0000_0001;
const SLEEP_MODE_MASK_HIGH: u8 = 0b0000_0010;
const SLEEP_MODE_OFFSET: u8 = 1;

#[derive(Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct LowByte(u8);

impl LowByte {
    #[must_use]
    pub const fn is_command(self) -> bool {
        self.0 & COMMAND_MASK == 0
    }

    #[must_use]
    pub const fn is_response(self) -> bool {
        !self.is_command()
    }

    #[must_use]
    pub const fn network_index(self) -> u8 {
        ((self.0 & NETWORK_INDEX_MASK_HIGH) >> NETWORK_INDEX_OFFSET_HIGH)
            + ((self.0 & NETWORK_INDEX_MASK_LOW) >> NETWORK_INDEX_OFFSET_LOW)
    }

    pub fn set_network_index(&mut self, network_index: u8) {
        let index = network_index & 0b11;

        if index != network_index {
            warn!("Network index truncated from {network_index:#04X} to {index:#04X}");
        }

        self.0 &= (0xFF ^ NETWORK_INDEX_MASK)
            | (((index & (NETWORK_INDEX_MASK_HIGH >> NETWORK_INDEX_OFFSET_HIGH))
                << NETWORK_INDEX_OFFSET_HIGH)
                + ((index & (NETWORK_INDEX_MASK_LOW >> NETWORK_INDEX_OFFSET_LOW))
                    << NETWORK_INDEX_OFFSET_LOW));
    }

    #[must_use]
    pub fn callback_type(self) -> Option<CallbackType> {
        CallbackType::from_u8(
            ((self.0 & CALLBACK_TYPE_MASK_HIGH) >> CALLBACK_OFFSET_HIGH)
                + ((self.0 & CALLBACK_TYPE_MASK_LOW) >> CALLBACK_OFFSET_LOW),
        )
    }

    #[must_use]
    pub const fn callback_pending(self) -> bool {
        self.0 & CALLBACK_PENDING_MASK != 0
    }

    #[must_use]
    pub const fn is_truncated(self) -> bool {
        self.0 & TRUNCATED_MASK != 0
    }

    #[must_use]
    pub const fn is_overflow(self) -> bool {
        self.0 & OVERFLOW_MASK != 0
    }

    #[must_use]
    pub fn sleep_mode(self) -> Option<SleepMode> {
        SleepMode::from_u8(
            ((self.0 & SLEEP_MODE_MASK_LOW) << SLEEP_MODE_OFFSET)
                + ((self.0 & SLEEP_MODE_MASK_HIGH) >> SLEEP_MODE_OFFSET),
        )
    }
}

impl From<LowByte> for u8 {
    fn from(low_byte: LowByte) -> Self {
        low_byte.0
    }
}

impl From<u8> for LowByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl FromLeBytes for LowByte {
    fn from_le_bytes<T>(bytes: &mut T) -> le_stream::Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        <u8 as FromLeBytes>::from_le_bytes(bytes).map(Self)
    }
}

impl ToLeBytes for LowByte {
    type Iter = Once<u8>;

    fn to_le_bytes(self) -> Self::Iter {
        once(self.0)
    }
}
