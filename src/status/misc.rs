use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Misc {
    Success = 0x00,
    AshInProgress = 0x20,
    HostFatalError = 0x21,
    AshNcpFatalError = 0x22,
    DataFrameTooLong = 0x23,
    DataFrameTooShort = 0x24,
    NoTxSpace = 0x25,
    NoRxSpace = 0x26,
    NoRxData = 0x27,
    NotConnected = 0x28,
    CpcErrorInit = 0x86,
    NoError = 0xFF,
}

impl From<Misc> for u8 {
    fn from(misc: Misc) -> Self {
        misc.to_u8().expect("could not convert Misc to u8")
    }
}
