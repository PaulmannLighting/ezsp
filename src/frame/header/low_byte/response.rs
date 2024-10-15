mod callback_type;

use bitflags::bitflags;
pub use callback_type::CallbackType;
use le_stream::derive::{FromLeStream, ToLeStream};

/// Low byte control field of the frame header when it represents a response.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Response(u8);

bitflags! {
    impl Response: u8 {
        /// Response flag. Should not be set.
        const IS_RESPONSE = 0b1000_0000;
        /// Network index bit no. 1.
        const NETWORK_INDEX_1 = 0b0100_0000;
        /// Network index bit no. 2.
        const NETWORK_INDEX_0 = 0b0010_0000;
        /// Callback type bit no. 1.
        const CALLBACK_TYPE_1 = 0b0001_0000;
        /// Callback type bit no. 2.
        const CALLBACK_TYPE_0 = 0b0000_1000;
        /// Callback pending flag.
        const CALLBACK_PENDING = 0b0000_0100;
        /// Truncated flag.
        const TRUNCATED = 0b0000_0010;
        /// Overflow flag.
        const OVERFLOW = 0b0000_0001;
    }
}

impl Response {
    /// Returns `true` if the response is a response else `false`.
    pub const fn is_response(self) -> bool {
        self.contains(Self::IS_RESPONSE)
    }

    /// Returns the network index.
    pub fn network_index(self) -> u8 {
        (self.bits() & (Self::NETWORK_INDEX_1 | Self::NETWORK_INDEX_0).bits()) >> 5
    }

    /// Returns the callback type.
    ///
    /// Returns `None` if this is not a callback.
    pub const fn callback_type(self) -> Option<CallbackType> {
        match (
            self.contains(Self::CALLBACK_TYPE_1),
            self.contains(Self::CALLBACK_TYPE_0),
        ) {
            (true, true) => Some(CallbackType::Reserved),
            (true, false) => Some(CallbackType::Async),
            (false, true) => Some(CallbackType::Sync),
            (false, false) => None,
        }
    }

    /// Returns `true` if a callback is pending on the NCP.
    ///
    /// If this response is a callback, at least one more callback is available.
    pub const fn is_callback_pending(self) -> bool {
        self.contains(Self::CALLBACK_PENDING)
    }

    /// Returns `true` if the response is truncated else `false`.
    pub const fn is_truncated(self) -> bool {
        self.contains(Self::TRUNCATED)
    }

    /// Returns `true` if the response has overflowed else `false`.
    ///
    /// This is used to indicate that the NCP truncated the current response
    /// to avoid exceeding the maximum EZSP frame length.
    pub const fn has_overflowed(self) -> bool {
        self.contains(Self::OVERFLOW)
    }
}
