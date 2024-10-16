//! Zigbee Light Link (ZLL) specific types.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// EZSP ZLL network operation.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum NetworkOperation {
    /// ZLL form network command.
    FormNetwork = 0x00,
    /// ZLL join target command.
    JoinTarget = 0x01,
}

impl From<NetworkOperation> for u8 {
    fn from(network_operation: NetworkOperation) -> Self {
        network_operation as Self
    }
}

impl TryFrom<u8> for NetworkOperation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
