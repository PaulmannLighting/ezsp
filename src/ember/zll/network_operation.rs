use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum NetworkOperation {
    FormNetwork = 0x00,
    JoinTarget = 0x01,
}

impl From<NetworkOperation> for u8 {
    fn from(network_operation: NetworkOperation) -> Self {
        network_operation
            .to_u8()
            .expect("NetworkOperation should always be convertible to u8.")
    }
}

impl TryFrom<u8> for NetworkOperation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
