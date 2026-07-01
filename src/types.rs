//! Types used in the `EZSP` protocol.

pub use self::source_route_discovery_mode::SourceRouteDiscoveryMode;
pub use self::variable_length_u32::VariableLengthU32;

mod source_route_discovery_mode;
mod variable_length_u32;

/// A vector with a maximum of 255 elements.
pub type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }, u8>;
