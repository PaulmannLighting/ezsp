//! Structures used in scan for available networks.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Indicates the type of scan to be performed.
///
/// Possible values are: [`Type::EnergyScan`] and [`Type::ActiveScan`].
///
/// For each type, the respective callback for reporting results is
/// [`EnergyScanResult`](crate::parameters::networking::handler::Handler::EnergyScanResult) and
/// [`NetworkFound`](crate::parameters::networking::handler::Handler::NetworkFound).
///
/// The energy scan and active scan report errors and completion via
/// [`ScanComplete`](crate::parameters::networking::handler::Handler::ScanComplete).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Type {
    /// An energy scan scans each channel for its RSSI value.
    EnergyScan = 0x00,
    /// An active scan scans each channel for available networks.
    ActiveScan = 0x01,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ as Self
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
