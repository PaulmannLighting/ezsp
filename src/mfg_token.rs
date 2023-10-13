mod manufacturing;
mod stack;

use manufacturing::Manufacturing;
use num_traits::{FromPrimitive, ToPrimitive};
use stack::Stack;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Id {
    Mfg(Manufacturing),
    Stack(Stack),
}

impl FromPrimitive for Id {
    fn from_i64(n: i64) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Manufacturing::from_u64(n)
            .map(Self::Mfg)
            .or_else(|| Stack::from_u64(n).map(Self::Stack))
    }
}

impl ToPrimitive for Id {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Mfg(manufacturing) => manufacturing.to_i64(),
            Self::Stack(stack) => stack.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::Mfg(manufacturing) => manufacturing.to_u64(),
            Self::Stack(stack) => stack.to_u64(),
        }
    }
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        id.to_u8().expect("could not convert Id to u8")
    }
}
