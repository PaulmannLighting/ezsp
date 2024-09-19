mod high_byte;
mod low_byte;

pub use high_byte::{FrameFormatVersion, HighByte};
use le_stream::derive::{FromLeStream, ToLeStream};
pub use low_byte::{CallbackType, LowByte, SleepMode};
use num_traits::ToBytes;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Control {
    low: LowByte,
    high: HighByte,
}

impl Control {
    #[must_use]
    pub const fn new(low: LowByte, high: HighByte) -> Self {
        Self { low, high }
    }

    #[must_use]
    pub const fn low(&self) -> LowByte {
        self.low
    }

    #[must_use]
    pub const fn high(&self) -> HighByte {
        self.high
    }
}

impl From<Control> for [u8; 2] {
    fn from(control: Control) -> Self {
        [control.low.into(), control.high.into()]
    }
}

impl From<[u8; 2]> for Control {
    fn from([low, high]: [u8; 2]) -> Self {
        Self::new(low.into(), high.into())
    }
}

impl From<u8> for Control {
    fn from(low: u8) -> Self {
        Self::new(low.into(), 0.into())
    }
}

impl From<u16> for Control {
    fn from(bytes: u16) -> Self {
        Self::from(bytes.to_le_bytes())
    }
}

impl From<Control> for u8 {
    fn from(control: Control) -> Self {
        control.low.into()
    }
}

impl From<Control> for u16 {
    fn from(control: Control) -> Self {
        u16::from_le_bytes(control.into())
    }
}
