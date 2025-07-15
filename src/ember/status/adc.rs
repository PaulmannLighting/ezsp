use std::error::Error;
use std::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

use super::values::Values;

/// Ember ADC status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Adc {
    /// Conversion is complete.
    ConversionDone,
    /// Conversion cannot be done because a request is being processed.
    ConversionBusy,
    /// Conversion is deferred until the current request has been processed.
    ConversionDeferred,
    /// No results are pending.
    NoConversionPending,
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

impl Error for Adc {}

impl From<Adc> for Values {
    fn from(adc: Adc) -> Self {
        match adc {
            Adc::ConversionDone => Self::AdcConversionDone,
            Adc::ConversionBusy => Self::AdcConversionBusy,
            Adc::ConversionDeferred => Self::AdcConversionDeferred,
            Adc::NoConversionPending => Self::AdcNoConversionPending,
        }
    }
}

impl From<Adc> for u8 {
    fn from(adc: Adc) -> Self {
        Values::from(adc).into()
    }
}

impl TryFrom<Values> for Adc {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::AdcConversionDone => Ok(Self::ConversionDone),
            Values::AdcConversionBusy => Ok(Self::ConversionBusy),
            Values::AdcConversionDeferred => Ok(Self::ConversionDeferred),
            Values::AdcNoConversionPending => Ok(Self::NoConversionPending),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Adc {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).and_then(|value| Self::try_from(value).ok())
    }
}
