use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Phy {
    TxUnderflow = 0x88,
    TxIncomplete = 0x89,
    InvalidChannel = 0x8A,
    InvalidPower = 0x8B,
    TxBusy = 0x8C,
    TxCcaFail = 0x8D,
    OscillatorCheckFailed = 0x8E,
    AckReceived = 0x8F,
}
