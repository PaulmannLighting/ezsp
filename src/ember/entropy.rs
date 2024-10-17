//! Entropy source.

use num_derive::FromPrimitive;

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
