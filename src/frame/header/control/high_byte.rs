use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

pub use frame_format_version::FrameFormatVersion;

use crate::frame::header::control::high_byte::frame_format_version::{
    bit_swap, FRAME_FORMAT_VERSION_MASK_HIGH, FRAME_FORMAT_VERSION_MASK_LOW,
};

mod frame_format_version;

const FRAME_FORMAT_VERSION_MASK: u8 =
    FRAME_FORMAT_VERSION_MASK_LOW + FRAME_FORMAT_VERSION_MASK_HIGH;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, FromLeStream, ToLeStream)]
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
        Self(bit_swap(FrameFormatVersion::One.into()))
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

#[cfg(test)]
mod tests {
    use super::{FrameFormatVersion, HighByte};

    #[test]
    fn test_default() {
        let high_byte = HighByte::default();
        let byte: u8 = high_byte.into();
        assert_eq!(byte, 0b0000_0001);
    }

    #[test]
    fn test_set_frame_format_version() {
        let mut high_byte = HighByte::default();

        high_byte.set_frame_format_version(FrameFormatVersion::One);
        let byte: u8 = high_byte.into();
        assert_eq!(byte, 0b0000_0001);

        high_byte.set_frame_format_version(FrameFormatVersion::Zero);
        let byte: u8 = high_byte.into();
        assert_eq!(byte, 0b0000_0000);
    }

    #[test]
    fn get_frame_format_version() {
        let high_byte = HighByte(0b0000_0000);
        assert_eq!(
            high_byte.frame_format_version(),
            Some(FrameFormatVersion::Zero)
        );

        let high_byte = HighByte(0b0000_0001);
        assert_eq!(
            high_byte.frame_format_version(),
            Some(FrameFormatVersion::One)
        );

        let high_byte = HighByte(0b0000_0010);
        assert_eq!(high_byte.frame_format_version(), None);

        let high_byte = HighByte(0b0000_0011);
        assert_eq!(high_byte.frame_format_version(), None);
    }
}
