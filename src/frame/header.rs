use std::fmt::Debug;
use std::hash::Hash;

pub use extended::Extended;
pub use high_byte::{FormatVersion, HighByte};
pub use legacy::Legacy;
pub use low_byte::{CallbackType, Command, LowByte, SleepMode};

mod extended;
mod high_byte;
mod legacy;
mod low_byte;

/// A trait to represent the header of a frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Header {
    Legacy(Legacy),
    Extended(Extended),
}

impl Header {
    /// Returns the header's frame ID as an `u16`,
    #[must_use]
    pub fn id(self) -> u16 {
        match self {
            Self::Legacy(legacy) => u16::from(legacy.id()),
            Self::Extended(extended) => extended.id(),
        }
    }

    #[must_use]
    pub fn is_async_callback(self) -> bool {
        if let Self::Extended(header) = self {
            if let LowByte::Response(response) = header.low_byte() {
                return response.callback_type() == Some(CallbackType::Async);
            }
        }

        false
    }
}
