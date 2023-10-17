use crate::frame::header::{Control, Header, LegacyHeader, HEADER_SIZE, LEGACY_HEADER_SIZE};
use anyhow::anyhow;
use std::io::{Read, Write};

pub mod configuration;
pub mod header;
pub mod utilities;

pub trait Parameters<T>: IntoIterator<Item = u8> + Sized {
    const FRAME_ID: T;

    /// Read parameters from a reader
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors.
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read;

    /// Write parameters to a writer
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on errors.
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        dst.write_all(&self.into_iter().collect::<Vec<_>>())
    }
}

#[derive(Debug)]
pub struct Frame<P>
where
    P: Parameters<u16>,
{
    header: Header,
    parameters: P,
}

impl<P> Frame<P>
where
    P: Parameters<u16>,
{
    pub const fn new(sequence: u8, control: Control, parameters: P) -> Self {
        Self {
            header: Header::new(sequence, control, P::FRAME_ID),
            parameters,
        }
    }

    /// Returns the header
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Returns the payload
    pub const fn parameters(&self) -> &P {
        &self.parameters
    }

    /// Writes the frame to a writer
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on errors.
    pub fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        let bytes: Vec<u8> = self.into();
        dst.write_all(&bytes)
    }
}

impl<P> From<Frame<P>> for Vec<u8>
where
    P: Parameters<u16>,
{
    fn from(frame: Frame<P>) -> Self {
        let header: [u8; HEADER_SIZE] = frame.header.into();
        let mut bytes = Self::from(header);
        bytes.extend(frame.parameters);
        bytes
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct LegacyFrame<P>
where
    P: Parameters<u8>,
{
    header: LegacyHeader,
    parameters: P,
}

impl<P> LegacyFrame<P>
where
    P: Parameters<u8>,
{
    pub const fn new(sequence: u8, control: u8, parameters: P) -> Self {
        Self {
            header: LegacyHeader::new(sequence, control, P::FRAME_ID),
            parameters,
        }
    }

    /// Returns the header
    pub const fn header(&self) -> &LegacyHeader {
        &self.header
    }

    /// Returns the parameters
    pub const fn parameters(&self) -> &P {
        &self.parameters
    }
}

impl<P> From<LegacyFrame<P>> for Vec<u8>
where
    P: Parameters<u8>,
{
    fn from(frame: LegacyFrame<P>) -> Self {
        let header: [u8; LEGACY_HEADER_SIZE] = frame.header.into();
        let mut bytes = Self::from(header);
        bytes.extend(frame.parameters);
        bytes
    }
}

pub enum Response {
    AddEndpoint(Frame<configuration::add_endpoint::Response>),
    GetConfigurationValue(Frame<configuration::get_configuration_value::Response>),
    // TODO: implement
}

impl Response {
    /// Read a response from a reader
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors.
    pub fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Header::read_from(src)?;

        match header.id() {
            configuration::add_endpoint::ID => Ok(Self::AddEndpoint(Frame {
                header,
                parameters: configuration::add_endpoint::Response::read_from(src)?,
            })),
            configuration::get_configuration_value::ID => Ok(Self::GetConfigurationValue(Frame {
                header,
                parameters: { configuration::get_configuration_value::Response::read_from(src)? },
            })),
            id => Err(anyhow!("invalid frame ID: {id}")),
        }
    }
}

pub enum LegacyResponse {
    Version(LegacyFrame<configuration::version::Response>),
}

impl LegacyResponse {
    /// Read a legacy response from a reader
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors.
    pub fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = LegacyHeader::read_from(src)?;

        match header.id() {
            configuration::version::ID => Ok(Self::Version(LegacyFrame {
                header,
                parameters: configuration::version::Response::read_from(src)?,
            })),
            id => Err(anyhow!("invalid frame ID: {id}")),
        }
    }
}
