use crate::frame::header::{Header, LegacyHeader, HEADER_SIZE, LEGACY_HEADER_SIZE};
use anyhow::anyhow;
use std::io::{Read, Write};

pub mod configuration;
pub mod header;
pub mod utilities;

pub trait Frame<const ID: u16>
where
    Self::Parameters: AsRef<[u8]>,
{
    type Parameters;

    /// Returns the frame ID
    #[must_use]
    fn id() -> u16 {
        ID
    }

    /// Returns the header
    fn header(&self) -> &Header;

    /// Returns the parameters as bytes
    fn parameters(&self) -> Option<Self::Parameters>;

    /// Reads a frame
    fn read_header<R>(reader: &mut R) -> anyhow::Result<Header>
    where
        R: Read,
        Self: Sized,
    {
        let header = Header::read_from(reader)?;

        if header.id() == ID {
            Ok(header)
        } else {
            Err(anyhow!("Frame ID mismatch: {} != {ID}", header.id()))
        }
    }

    fn read_from<R>(reader: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
        Self: Sized;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(<[u8; HEADER_SIZE]>::from(self.header()));

        if let Some(parameters) = self.parameters() {
            bytes.extend_from_slice(parameters.as_ref());
        }

        bytes
    }

    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        writer.write_all(&self.to_bytes())
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait LegacyFrame<const ID: u8>
where
    Self::Parameters: AsRef<[u8]>,
{
    type Parameters;

    /// Returns the frame ID
    #[must_use]
    fn id() -> u8 {
        ID
    }

    /// Returns the header
    fn header(&self) -> &LegacyHeader;

    /// Returns the parameters as bytes
    fn parameters(&self) -> Option<Self::Parameters>;

    /// Reads a frame
    fn read_header<R>(reader: &mut R) -> anyhow::Result<LegacyHeader>
    where
        R: Read,
        Self: Sized,
    {
        let header = LegacyHeader::read_from(reader)?;

        if header.id() == ID {
            Ok(header)
        } else {
            Err(anyhow!("Frame ID mismatch: {} != {ID}", header.id()))
        }
    }

    fn read_from<R>(reader: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
        Self: Sized;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(<[u8; LEGACY_HEADER_SIZE]>::from(self.header()));

        if let Some(parameters) = self.parameters() {
            bytes.extend_from_slice(parameters.as_ref());
        }

        bytes
    }

    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        writer.write_all(&self.to_bytes())
    }
}
