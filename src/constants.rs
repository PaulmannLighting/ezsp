/// And `EZSP` header has a maximum size of 5 bytes.
///
/// Legacy headers are 3 (three) bytes of size, while the newer ones are 5 (five) bytes.
pub const MAX_HEADER_SIZE: usize = 5;

/// The largest parameter size that is transmitted by the `EZSP` protocol.
///
/// This is the maximum size of the payload of the frame, not including the header.
///
/// The largest parameter that we've found in the protocol definition is the `addEndpoint` parameter.
/// Its size amounts to 1028 bytes:
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

/// The largest frame size that can be sent or received by the `EZSP` protocol.
pub const MAX_FRAME_SIZE: usize = MAX_PARAMETER_SIZE + MAX_HEADER_SIZE;
