mod frame_format_version;

use crate::frame::header::control::high_byte::frame_format_version::{
    bit_swap, FRAME_FORMAT_VERSION_MASK_HIGH, FRAME_FORMAT_VERSION_MASK_LOW,
};
pub use frame_format_version::FrameFormatVersion;
use num_traits::FromPrimitive;

const FRAME_FORMAT_VERSION_MASK: u8 =
    FRAME_FORMAT_VERSION_MASK_LOW + FRAME_FORMAT_VERSION_MASK_HIGH;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct HighByte(u8);

impl HighByte {
    #[must_use]
    pub fn frame_format_version(self) -> Option<FrameFormatVersion> {
        FrameFormatVersion::from_u8(bit_swap(self.0 & FRAME_FORMAT_VERSION_MASK))
    }

    pub fn set_frame_format_version(&mut self, version: FrameFormatVersion) {
        self.0 &= (0xFF ^ FRAME_FORMAT_VERSION_MASK) | bit_swap(version.into());
    }
}

impl Default for HighByte {
    fn default() -> Self {
        Self(FrameFormatVersion::One.into())
    }
}

impl From<HighByte> for u8 {
    fn from(low_byte: HighByte) -> Self {
        low_byte.0
    }
}

impl From<u8> for HighByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
