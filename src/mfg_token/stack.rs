use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Stack {
    CalData = 0x08,
    CalFilter = 0x0B,
}

impl From<Stack> for u8 {
    fn from(stack: Stack) -> Self {
        stack.to_u8().expect("could not convert Stack to u8")
    }
}
