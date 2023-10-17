use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Application {
    Error0 = 0xF0,
    Error1 = 0xF1,
    Error2 = 0xF2,
    Error3 = 0xF3,
    Error4 = 0xF4,
    Error5 = 0xF5,
    Error6 = 0xF6,
    Error7 = 0xF7,
    Error8 = 0xF8,
    Error9 = 0xF9,
    Error10 = 0xFA,
    Error11 = 0xFB,
    Error12 = 0xFC,
    Error13 = 0xFD,
    Error14 = 0xFE,
    Error15 = 0xFF,
}
