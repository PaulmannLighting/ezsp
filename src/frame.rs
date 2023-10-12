use crate::frame::header::{Control, Header, LegacyHeader};

pub mod configuration;
mod header;
mod utilities;

pub trait Frame<const ID: u16>
where
    Self::Parameters: AsRef<[u8]>,
{
    type Parameters;

    /// Returns the frame ID
    fn id() -> u16 {
        ID
    }

    /// Returns the header
    fn header(&self) -> &Header;

    /// Returns the parameters as bytes
    fn parameters(&self) -> Self::Parameters;

    /// Creates a new header for the frame
    fn make_header(sequence: u8, control: Control) -> Header {
        Header::new(sequence, control, ID)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header.into());
        bytes.extend_from_slice(self.parameters().as_ref());
        bytes
    }
}

pub trait LegacyFrame<const ID: u8>
where
    Self::Parameters: AsRef<[u8]>,
{
    type Parameters;

    /// Returns the frame ID
    fn id() -> u8 {
        ID
    }

    /// Returns the header
    fn header(&self) -> &LegacyHeader;

    /// Returns the parameters as bytes
    fn parameters(&self) -> Self::Parameters;

    /// Creates a new header for the frame
    fn make_header(sequence: u8, control: u8) -> LegacyHeader {
        LegacyHeader::new(sequence, control, ID)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header.into());
        bytes.extend_from_slice(self.parameters().as_ref());
        bytes
    }
}
