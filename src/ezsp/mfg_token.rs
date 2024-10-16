//! Manufacturing tokens.

use num_traits::FromPrimitive;

pub use mfg::Mfg;
pub use stack::Stack;

mod mfg;
mod stack;

/// Manufacturing token IDs used by `ezspGetMfgToken()`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Id {
    /// IDs pertaining to the manufacturer.
    Mfg(Mfg),
    /// IDs pertaining to the stack.
    Stack(Stack),
}

impl From<Id> for u8 {
    fn from(id: Id) -> Self {
        match id {
            Id::Mfg(manufacturing) => manufacturing.into(),
            Id::Stack(stack) => stack.into(),
        }
    }
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

impl TryFrom<u8> for Id {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
