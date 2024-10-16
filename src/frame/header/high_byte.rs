mod frame_format_version;

use bitflags::bitflags;
pub use frame_format_version::FrameFormatVersion;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The extended frame control field of the frame header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct HighByte(u8);

bitflags! {
    impl HighByte: u8 {
        /// Security enabled flag.
        const SECURITY_ENABLED = 0b1000_0000;
        /// Padding enabled flag.
        const PADDING_ENABLED = 0b0100_0000;
        /// Frame format version bit no. 1.
        const FRAME_FORMAT_VERSION_1 = 0b0000_0010;
        /// Frame format version bit no. 2.
        const FRAME_FORMAT_VERSION_0 = 0b0000_0001;
    }
}

impl HighByte {
    /// Returns `true` if security is enabled else `false`.
    pub const fn is_security_enabled(self) -> bool {
        self.contains(Self::SECURITY_ENABLED)
    }

    /// Returns `true` if padding is enabled else `false`.
    pub const fn is_padding_enabled(self) -> bool {
        self.contains(Self::PADDING_ENABLED)
    }

    /// Returns the frame format version.
    pub const fn frame_format_version(self) -> FrameFormatVersion {
        match (
            self.contains(Self::FRAME_FORMAT_VERSION_1),
            self.contains(Self::FRAME_FORMAT_VERSION_0),
        ) {
            (true, _) => FrameFormatVersion::Reserved,
            (false, true) => FrameFormatVersion::One,
            (false, false) => FrameFormatVersion::Zero,
        }
    }
}

impl Default for HighByte {
    fn default() -> Self {
        Self::FRAME_FORMAT_VERSION_0
    }
}
