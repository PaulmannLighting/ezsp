use crate::frame::header::control::high_byte::HighByte;
use crate::frame::header::control::low_byte::LowByte;

pub mod high_byte;
pub mod low_byte;

#[derive(Debug, Default, Eq, PartialEq)]
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
