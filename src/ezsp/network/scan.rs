use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// Indicates the type of scan to be performed.
///
/// Possible values are: [`Type::EnergyScan`] and [`Type::ActiveScan`].
/// For each type, the respective callback for reporting results is: energyScanResultHandler and networkFoundHandler.
/// The energy scan and active scan report errors and completion via the scanCompleteHandler.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Type {
    EnergyScan = 0x00,
    ActiveScan = 0x01,
}

impl From<Type> for u8 {
    fn from(typ: Type) -> Self {
        typ.to_u8().expect("could not convert Type to u8")
    }
}

impl TryFrom<u8> for Type {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
