const HEADER_SIZE: usize = 5;
const LEGACY_HEADER_SIZE: usize = 3;

#[derive(Debug, Eq, PartialEq)]
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

impl From<[u8; HEADER_SIZE]> for Header {
    fn from(bytes: [u8; HEADER_SIZE]) -> Self {
        Self::new(
            bytes[0],
            u16::from_be_bytes([bytes[1], bytes[2]]),
            u16::from_be_bytes([bytes[3], bytes[4]]),
        )
    }
}

impl From<Header> for [u8; HEADER_SIZE] {
    fn from(header: Header) -> Self {
        let control = header.control.to_be_bytes();
        let id = header.id.to_be_bytes();
        [header.sequence, control[0], control[1], id[0], id[1]]
    }
}

#[derive(Debug, Eq, PartialEq)]
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

impl From<[u8; LEGACY_HEADER_SIZE]> for LegacyHeader {
    fn from(bytes: [u8; LEGACY_HEADER_SIZE]) -> Self {
        Self::new(bytes[0], bytes[1], bytes[2])
    }
}

impl From<LegacyHeader> for [u8; LEGACY_HEADER_SIZE] {
    fn from(header: LegacyHeader) -> Self {
        [header.sequence, header.control, header.id]
    }
}
