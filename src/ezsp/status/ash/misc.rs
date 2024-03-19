use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Misc {
    Started = 0x70,
    Connected = 0x71,
    Disconnected = 0x72,
    AckTimeout = 0x73,
    Cancelled = 0x74,
    OutOfSequence = 0x75,
    BadCrc = 0x76,
    CommError = 0x77,
    BadAckNum = 0x78,
    TooShort = 0x79,
    TooLong = 0x7A,
    BadControl = 0x7B,
    BadLength = 0x7C,
    AckReceived = 0x7D,
    AckSent = 0x7E,
    NakReceived = 0x7F,
    NakSent = 0x80,
    RstReceived = 0x81,
    RstSent = 0x82,
    Status = 0x83,
    Tx = 0x84,
    Rx = 0x85,
}

impl From<Misc> for u8 {
    fn from(misc: Misc) -> Self {
        misc.to_u8()
            .expect("Misc should always be convertible to u8.")
    }
}
