use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum SimEeprom {
    ErasePageGreen = 0x43,
    ErasePageRed = 0x44,
    Full = 0x45,
    Init1Failed = 0x48,
    Init2Failed = 0x49,
    Init3Failed = 0x4A,
}
