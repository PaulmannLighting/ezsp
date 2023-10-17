use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Bootloader {
    TrapTableBad = 0x58,
    TrapUnknown = 0x59,
    NoImage = 0x5A,
}
