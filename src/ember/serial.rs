use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Serial {
    InvalidBaudRate = 0x20,
    InvalidPort = 0x21,
    TxOverflow = 0x22,
    RxOverflow = 0x23,
    RxFrameError = 0x24,
    RxParityError = 0x025,
    RxEmpty = 0x26,
    RxOverrunError = 0x27,
}
