use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember ADC status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Adc {
    /// Conversion is complete.
    ConversionDone = 0x80,
    /// Conversion cannot be done because a request is being processed.
    ConversionBusy = 0x81,
    /// Conversion is deferred until the current request has been processed.
    ConversionDeferred = 0x82,
    /// No results are pending.
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
