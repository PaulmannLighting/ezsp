use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
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
