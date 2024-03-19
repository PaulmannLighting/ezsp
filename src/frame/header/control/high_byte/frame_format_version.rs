use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

pub const FRAME_FORMAT_VERSION_MASK_LOW: u8 = 0b0000_0001;
pub const FRAME_FORMAT_VERSION_MASK_HIGH: u8 = 0b0000_0010;
const FRAME_FORMAT_VERSION_OFFSET: u8 = 1;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum FrameFormatVersion {
    One = 0b10,
    Zero = 0b00,
}

impl From<FrameFormatVersion> for u8 {
    fn from(frame_format_version: FrameFormatVersion) -> Self {
        frame_format_version
            .to_u8()
            .expect("FrameFormatVersion should always be convertible to u8.")
    }
}

#[must_use]
pub const fn bit_swap(frame_format_version: u8) -> u8 {
    ((frame_format_version & FRAME_FORMAT_VERSION_MASK_LOW) << FRAME_FORMAT_VERSION_OFFSET)
        + ((frame_format_version & FRAME_FORMAT_VERSION_MASK_HIGH) >> FRAME_FORMAT_VERSION_OFFSET)
}
