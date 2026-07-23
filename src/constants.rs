use std::num::NonZero;

/// Maximum encoded EZSP header size in bytes.
///
/// Legacy headers are three bytes: sequence, frame control, and frame ID. EZSP
/// protocol version 8 and newer use the extended five-byte format: sequence,
/// two frame-control bytes, and a two-byte frame ID.
pub const MAX_HEADER_SIZE: usize = 5;

/// Minimum EZSP protocol version that uses the extended header format.
///
/// Protocol versions are nonzero, so this constant can also be used directly
/// as the default version requested during connection negotiation.
pub const MIN_NON_LEGACY_VERSION: NonZero<u8> = NonZero::new(8).expect("Non-zero.");
