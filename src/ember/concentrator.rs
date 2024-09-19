use std::num::TryFromIntError;
use std::time::Duration;

use le_stream::derive::ToLeBytes;

/// Concentrator parameters.
#[derive(Clone, Debug, Default, Eq, PartialEq, ToLeBytes)]
pub struct Parameters {
    concentrator_type: u16,
    min_time: u16,
    max_time: u16,
    route_error_threshold: u8,
    delivery_failure_threshold: u8,
    max_hops: u8,
}

impl Parameters {
    /// Create a new `Parameters` instance.
    ///
    /// # Errors
    /// Returns a [`TryFromIntError`] if the `min_time` or `max_time` values are too large to fit.
    pub fn new(
        concentrator_type: Type,
        min_time: Duration,
        max_time: Duration,
        route_error_threshold: u8,
        delivery_failure_threshold: u8,
        max_hops: u8,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            concentrator_type: concentrator_type as u16,
            min_time: min_time.as_secs().try_into()?,
            max_time: max_time.as_secs().try_into()?,
            route_error_threshold,
            delivery_failure_threshold,
            max_hops,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum Type {
    LowRam = 0xFFF8,
    HighRam = 0xFFF9,
}
