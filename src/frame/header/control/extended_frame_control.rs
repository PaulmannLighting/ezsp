mod frame_format_version;

use bitflags::bitflags;
pub use frame_format_version::FrameFormatVersion;
use le_stream::derive::{FromLeStream, ToLeStream};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct ExtendedFrameControl(u8);

bitflags! {
    impl ExtendedFrameControl: u8 {
        const SECURITY_ENABLED = 0b1000_0000;
        const PADDING_ENABLED = 0b0100_0000;
        const FRAME_FORMAT_VERSION_1 = 0b0000_0010;
        const FRAME_FORMAT_VERSION_0 = 0b0000_0001;
    }
}

impl ExtendedFrameControl {
    /// Returns `true` if security is enabled else `false`.
    pub fn is_security_enabled(&self) -> bool {
        self.contains(Self::SECURITY_ENABLED)
    }

    /// Returns `true` if padding is enabled else `false`.
    pub fn is_padding_enabled(&self) -> bool {
        self.contains(Self::PADDING_ENABLED)
    }

    /// Returns the frame format version.
    pub fn frame_format_version(&self) -> FrameFormatVersion {
        match (
            self.contains(Self::FRAME_FORMAT_VERSION_1),
            self.contains(Self::FRAME_FORMAT_VERSION_0),
        ) {
            (true, true) => FrameFormatVersion::Reserved,
            (true, false) => FrameFormatVersion::Reserved,
            (false, true) => FrameFormatVersion::One,
            (false, false) => FrameFormatVersion::Zero,
        }
    }
}

impl Default for ExtendedFrameControl {
    fn default() -> Self {
        Self::FRAME_FORMAT_VERSION_0
    }
}
