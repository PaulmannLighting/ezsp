mod sleep_mode;

use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};
pub use sleep_mode::SleepMode;

/// Low byte control field of the frame header when it represents a command.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Command(u8);

bitflags! {
    impl Command: u8 {
        /// The response flag. Should be set.
        const IS_RESPONSE = 0b1000_0000;
        /// Network index bit no. 1.
        const NETWORK_INDEX_1 = 0b0100_0000;
        /// Network index bit no. 2.
        const NETWORK_INDEX_0 = 0b0010_0000;
        /// Sleep mode bit no. 1.
        const SLEEP_MODE_1 = 0b0000_0010;
        /// Sleep mode bit no. 2.
        const SLEEP_MODE_0 = 0b0000_0001;
    }
}

impl Command {
    /// Returns `true` if the command is a response else `false`.
    pub const fn is_response(self) -> bool {
        self.contains(Self::IS_RESPONSE)
    }

    /// Returns the network index.
    pub fn network_index(self) -> u8 {
        (self.bits() & (Self::NETWORK_INDEX_1 | Self::NETWORK_INDEX_0).bits()) >> 5
    }

    /// Returns the sleep mode.
    pub const fn sleep_mode(self) -> SleepMode {
        match (
            self.contains(Self::SLEEP_MODE_1),
            self.contains(Self::SLEEP_MODE_0),
        ) {
            (true, true) => SleepMode::Reserved,
            (true, false) => SleepMode::PowerDown,
            (false, true) => SleepMode::DeepSleep,
            (false, false) => SleepMode::Idle,
        }
    }
}
