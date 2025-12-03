use core::fmt::{self, Debug, Display};
use core::hash::Hash;

pub use self::extended::Extended;
pub use self::high_byte::{FormatVersion, HighByte};
pub use self::legacy::Legacy;
#[cfg(feature = "ashv2")]
pub use self::low_byte::Command;
pub use self::low_byte::{CallbackType, LowByte, SleepMode};

mod extended;
mod high_byte;
mod legacy;
mod low_byte;

/// Available header types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Header {
    /// A legacy header.
    Legacy(Legacy),
    /// An extended header.
    Extended(Extended),
}

impl Header {
    /// Returns the header's sequence number.
    #[must_use]
    pub const fn sequence(self) -> u8 {
        match self {
            Self::Legacy(legacy) => legacy.sequence(),
            Self::Extended(extended) => extended.sequence(),
        }
    }

    /// Returns the low byte.
    #[must_use]
    pub const fn low_byte(self) -> LowByte {
        match self {
            Self::Legacy(legacy) => legacy.low_byte(),
            Self::Extended(extended) => extended.low_byte(),
        }
    }

    /// Returns the header's frame ID as a `u16`,
    #[must_use]
    pub fn id(self) -> u16 {
        match self {
            Self::Legacy(legacy) => u16::from(legacy.id()),
            Self::Extended(extended) => extended.id(),
        }
    }

    /// Returns `true` if the header indicates an asynchronous callback else `false`.
    #[must_use]
    pub fn is_async_callback(self) -> bool {
        if let LowByte::Response(response) = self.low_byte() {
            response.callback_type() == Some(CallbackType::Async)
        } else {
            false
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Legacy(legacy) => write!(f, "Legacy({legacy})"),
            Self::Extended(extended) => write!(f, "Extended({extended})"),
        }
    }
}
