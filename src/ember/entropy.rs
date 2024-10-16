//! Entropy source.

use crate::resolve::Resolve;
use crate::ValueError;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember entropy source.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Source {
    /// Entropy source error.
    Error = 0,
    /// Entropy source is the radio.
    Radio = 1,
    /// Entropy source is the TRNG powered by mbed TLS.
    MbedTlsTRNG = 2,
    /// Entropy source is powered by mbed TLS, the source is not TRNG.
    MbedTls = 3,
}

impl From<Source> for u8 {
    fn from(source: Source) -> Self {
        source as Self
    }
}

impl TryFrom<u8> for Source {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl Resolve for Result<Source, u8> {
    type Output = Source;

    fn resolve(self) -> crate::Result<Self::Output> {
        match self {
            Ok(source) => Ok(source),
            Err(value) => Err(crate::Error::ValueError(ValueError::EntropySource(value))),
        }
    }
}
