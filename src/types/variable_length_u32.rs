use le_stream::{FromLeStream, ToLeStream};

#[expect(clippy::struct_field_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct VariableLengthU32 {
    byte_1: u8,
    byte_2: Option<u8>,
    byte_3: Option<u8>,
    byte_4: Option<u8>,
}

impl From<VariableLengthU32> for u32 {
    fn from(status: VariableLengthU32) -> Self {
        let mut result = Self::from(status.byte_1);

        if let Some(byte) = status.byte_2 {
            result |= Self::from(byte) << 8;
        }
        if let Some(byte) = status.byte_3 {
            result |= Self::from(byte) << 16;
        }
        if let Some(byte) = status.byte_4 {
            result |= Self::from(byte) << 24;
        }

        result
    }
}

impl From<u32> for VariableLengthU32 {
    fn from(value: u32) -> Self {
        let byte_1 = (value & 0xFF) as u8;

        let byte_2 = if value > 0xFF {
            Some(((value >> 8) & 0xFF) as u8)
        } else {
            None
        };

        let byte_3 = if value > 0xFFFF {
            Some(((value >> 16) & 0xFF) as u8)
        } else {
            None
        };

        let byte_4 = if value > 0x00FF_FFFF {
            Some(((value >> 24) & 0xFF) as u8)
        } else {
            None
        };

        Self {
            byte_1,
            byte_2,
            byte_3,
            byte_4,
        }
    }
}
