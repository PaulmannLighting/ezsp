mod frame_format_version;

pub use frame_format_version::FrameFormatVersion;
use num_traits::FromPrimitive;

const VERSION_1: u8 = 0b01;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct HighByte(u8);

impl HighByte {
    #[must_use]
    pub fn frame_format_version(self) -> Option<FrameFormatVersion> {
        FrameFormatVersion::from_u8(self.0 & 0b0000_0011)
    }

    pub fn set_frame_format_version(&mut self, version: FrameFormatVersion) {
        self.0 &= (0xFF ^ 0b000_0011) | <FrameFormatVersion as Into<u8>>::into(version);
    }
}

impl Default for HighByte {
    fn default() -> Self {
        Self(VERSION_1)
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
