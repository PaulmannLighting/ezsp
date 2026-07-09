/// Maximum encoded EZSP header size in bytes.
///
/// Legacy headers are three bytes: sequence, frame control, and frame ID. EZSP
/// protocol version 8 and newer use the extended five-byte format: sequence,
/// two frame-control bytes, and a two-byte frame ID.
pub const MAX_HEADER_SIZE: usize = 5;

/// Largest parameter body size handled by this implementation.
///
/// This is the maximum size of the EZSP parameters, not including the EZSP
/// header. This is a protocol model limit, not a UART fragmentation limit.
/// EZSP has no protocol-level fragmentation, and each UART EZSP frame must fit
/// in a single `ASHv2` DATA payload.
///
/// The largest parameter body in the referenced protocol definition is the
/// `addEndpoint` command:
///
/// - `endpoint` (1 byte)
/// - `profileId` (2 bytes)
/// - `deviceId` (2 bytes)
/// - `appFlags` (1 byte)
/// - `inputClusterCount` (1 byte)
/// - `outputClusterCount` (1 byte)
/// - `inputClusterList` (255 * 2 = 510 bytes)
/// - `outputClusterList` (255 * 2 = 510 bytes)
pub const MAX_PARAMETER_SIZE: usize = 1028;

/// Minimum EZSP protocol version that uses the extended header format.
pub const MIN_NON_LEGACY_VERSION: u8 = 8;
