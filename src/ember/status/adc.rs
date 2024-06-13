use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Adc {
    ConversionDone = 0x80,
    ConversionBusy = 0x81,
    ConversionDeferred = 0x82,
    NoConversionPending = 0x84,
}

impl Display for Adc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConversionDone => write!(f, "conversion done"),
            Self::ConversionBusy => write!(f, "conversion busy"),
            Self::ConversionDeferred => write!(f, "conversion deferred"),
            Self::NoConversionPending => write!(f, "no conversion pending"),
        }
    }
}

impl From<Adc> for u8 {
    fn from(adc: Adc) -> Self {
        adc as Self
    }
}

impl Error for Adc {}
