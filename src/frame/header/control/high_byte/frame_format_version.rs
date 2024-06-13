use num_derive::FromPrimitive;

pub const FRAME_FORMAT_VERSION_MASK_LOW: u8 = 0b0000_0001;
pub const FRAME_FORMAT_VERSION_MASK_HIGH: u8 = 0b0000_0010;
const FRAME_FORMAT_VERSION_OFFSET: u8 = 1;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum FrameFormatVersion {
    One = 0b10,
    Zero = 0b00,
}

impl From<FrameFormatVersion> for u8 {
    fn from(frame_format_version: FrameFormatVersion) -> Self {
        frame_format_version as Self
    }
}

#[must_use]
pub const fn bit_swap(frame_format_version: u8) -> u8 {
    ((frame_format_version & FRAME_FORMAT_VERSION_MASK_LOW) << FRAME_FORMAT_VERSION_OFFSET)
        + ((frame_format_version & FRAME_FORMAT_VERSION_MASK_HIGH) >> FRAME_FORMAT_VERSION_OFFSET)
}
