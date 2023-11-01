#[derive(Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct LowByte(u8);

impl From<LowByte> for u8 {
    fn from(low_byte: LowByte) -> Self {
        low_byte.0
    }
}

impl From<u8> for LowByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
