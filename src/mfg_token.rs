pub mod mfg;
pub mod stack;

use anyhow::anyhow;
use mfg::Mfg;
use num_traits::{FromPrimitive, ToPrimitive};
use stack::Stack;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Id {
    Mfg(Mfg),
    Stack(Stack),
}

impl FromPrimitive for Id {
    fn from_i64(n: i64) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Mfg::from_u64(n)
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

impl TryFrom<u8> for Id {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid Id: {value:#04X}"))
    }
}
