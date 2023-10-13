use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Error {
    Version = 0x50,
    Timeouts = 0x51,
    ResetFail = 0x52,
    NcpReset = 0x53,
    SerialInit = 0x54,
    NcpType = 0x55,
    ResetMethod = 0x56,
    XOnXOff = 0x57,
}
