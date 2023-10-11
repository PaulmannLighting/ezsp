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
        let [sequence, control_0, control_1, id_0, id_1] = bytes;
        Self::new(
            sequence,
            u16::from_be_bytes([control_0, control_1]),
            u16::from_be_bytes([id_0, id_1]),
        )
    }
}

impl From<Header> for [u8; HEADER_SIZE] {
    fn from(header: Header) -> Self {
        let [control_0, control_1] = header.control.to_be_bytes();
        let [id_0, id_1] = header.id.to_be_bytes();
        [header.sequence, control_0, control_1, id_0, id_1]
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
        let [sequence, control, id] = bytes;
        Self::new(sequence, control, id)
    }
}

impl From<LegacyHeader> for [u8; LEGACY_HEADER_SIZE] {
    fn from(header: LegacyHeader) -> Self {
        [header.sequence, header.control, header.id]
    }
}
