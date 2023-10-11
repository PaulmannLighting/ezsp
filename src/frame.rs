use crate::frame::header::{Header, LegacyHeader};

mod configuration;
mod header;

pub trait Frame<const ID: u16> {
    fn header(&self) -> &Header;

    fn make_header(sequence: u8, control: u16) -> Header {
        Header::new(sequence, control, Self::ID)
    }
}

pub trait LegacyFrame<const ID: u8> {
    fn header(&self) -> &LegacyHeader;

    fn make_header(sequence: u8, control: u8) -> LegacyHeader {
        LegacyHeader::new(sequence, control, Self::ID)
    }
}
