use std::mem::size_of;

pub struct Header {
    sequence: u8,
    control: u16,
    id: u16,
}

impl Header {
    pub const fn new(sequence: u8, control: u16, id: u16) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() == size_of::<Self>() {
            Ok(Self::new(
                bytes[0],
                u16::from_be_bytes([bytes[1], bytes[2]]),
                u16::from_be_bytes([bytes[3], bytes[4]]),
            ))
        } else {
            Err(Self::Error::InvalidSize {
                expected: size_of::<Self>(),
                found: bytes.len(),
            })
        }
    }
}

pub struct LegacyHeader {
    sequence: u8,
    control: u8,
    id: u8,
}

impl LegacyHeader {
    pub const fn new(sequence: u8, control: u8, id: u8) -> Self {
        Self {
            sequence,
            control,
            id,
        }
    }
}

impl TryFrom<&[u8]> for LegacyHeader {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() == size_of::<Self>() {
            Ok(Self::new(bytes[0], bytes[1], bytes[2]))
        } else {
            Err(Self::Error::InvalidSize {
                expected: size_of::<Self>(),
                found: bytes.len(),
            })
        }
    }
}
