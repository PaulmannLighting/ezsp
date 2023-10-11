use crate::frame::header::{Control, Header, LegacyHeader};

mod configuration;
mod header;

pub trait Frame<const ID: u16, const PARAMETERS: usize> {
    /// Returns the header
    fn header(&self) -> &Header;

    /// Returns the parameters as bytes
    fn parameters(&self) -> [u8; PARAMETERS];

    /// Creates a new header for the frame
    fn make_header(sequence: u8, control: Control) -> Header {
        Header::new(sequence, control, Self::ID)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header.into());
        bytes.extend_from_slice(&self.parameters());
        bytes
    }
}

pub trait LegacyFrame<const ID: u8, const PARAMETERS: usize> {
    /// Returns the header
    fn header(&self) -> &LegacyHeader;

    /// Returns the parameters as bytes
    fn parameters(&self) -> [u8; PARAMETERS];

    /// Creates a new header for the frame
    fn make_header(sequence: u8, control: u8) -> LegacyHeader {
        LegacyHeader::new(sequence, control, Self::ID)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header.into());
        bytes.extend_from_slice(&self.parameters());
        bytes
    }
}
