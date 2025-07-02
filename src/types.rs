//! Types used in the `EZSP` protocol.

pub use le_stream::ByteSizedVec;
use le_stream::derive::FromLeStream;

/// Discovery mode for source routes.
///
/// See p. 88 `setSourceRouteDiscoveryMode` of the `EZSP` specification.
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SourceRouteDiscoveryMode {
    /// Source route discovery is off.
    Off = 0,
    /// Source route discovery is on.
    On = 1,
    /// Source route discovery is set to reschedule.
    Reschedule = 2,
}

impl From<SourceRouteDiscoveryMode> for u8 {
    fn from(mode: SourceRouteDiscoveryMode) -> Self {
        mode as Self
    }
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct VariableLengthU32 {
    byte_1: u8,
    byte_2: Option<u8>,
    byte_3: Option<u8>,
    byte_4: Option<u8>,
}

impl From<VariableLengthU32> for u32 {
    fn from(status: VariableLengthU32) -> Self {
        let mut result = Self::from(status.byte_1);

        if let Some(byte) = status.byte_2 {
            result |= Self::from(byte) << 8;
        }
        if let Some(byte) = status.byte_3 {
            result |= Self::from(byte) << 16;
        }
        if let Some(byte) = status.byte_4 {
            result |= Self::from(byte) << 24;
        }

        result
    }
}
