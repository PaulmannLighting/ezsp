use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Stack {
    CalData = 0x08,
    CalFilter = 0x0B,
}

impl From<Stack> for u8 {
    fn from(stack: Stack) -> Self {
        stack as Self
    }
}
