use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum FrameFormatVersion {
    One = 0b10,
    Zero = 0b00,
}

impl From<FrameFormatVersion> for u8 {
    fn from(frame_format_version: FrameFormatVersion) -> Self {
        frame_format_version
            .to_u8()
            .expect("Could not convert frame format version to u8.")
    }
}
