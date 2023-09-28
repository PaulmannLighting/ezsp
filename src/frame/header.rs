pub struct Header {
    sequence: u8,
    control: u16,
    id: u16,
}

pub struct LegacyHeader {
    sequence: u8,
    control: u8,
    id: u8,
}
