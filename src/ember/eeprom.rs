use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Eeprom {
    MfgStackVersionMismatch = 0x04,
    MfgVersionMismatch = 0x06,
    StackVersionMismatch = 0x07,
}
