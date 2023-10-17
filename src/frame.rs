use crate::frame::header::{Control, Header, LegacyHeader};
use anyhow::anyhow;
use std::io::{Read, Write};

pub mod configuration;
pub mod header;
pub mod utilities;

pub trait Parameters<T>: IntoIterator<Item = u8> {
    const FRAME_ID: T;

    fn read_from<R>(src: R) -> anyhow::Result<Self>
    where
        R: Read;

    fn write_to<W>(&self, dst: &mut W) -> std::io::Result<()>
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

impl<P> Frame<P> {
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
    pub fn parameters(&self) -> &P {
        &self.parameters
    }

    /// Reads a frame
    pub fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Header::read_from(src)?;

        let parameters = match header.id() {
            configuration::add_endpoint::ID => {
                configuration::add_endpoint::Response::read_from(src)?
            }
            configuration::get_configuration_value::ID => {
                configuration::get_configuration_value::Response::read_from(src)?
            }
            id => return Err(anyhow!("invalid frame ID: {id}")),
        };

        Ok(Self { header, parameters })
    }

    pub fn write_to<W>(&self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        dst.write_all(&self.into())
    }
}

impl<P> From<Frame<P>> for Vec<u8> {
    fn from(frame: Frame<P>) -> Self {
        let mut bytes = frame.header.into();
        bytes.extend(frame.parameters.into_iter());
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
    pub fn parameters(&self) -> &P {
        &self.parameters
    }

    pub fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = LegacyHeader::read_from(src)?;

        let parameters = match header.id() {
            configuration::version::ID => configuration::version::Response::read_from(src)?,
            id => return Err(anyhow!("invalid frame ID: {id}")),
        };

        Ok(Self { header, parameters })
    }
}

impl<P> From<LegacyFrame<P>> for Vec<u8> {
    fn from(frame: LegacyFrame<P>) -> Self {
        let mut bytes = frame.header.into();
        bytes.extend(frame.parameters.into_iter());
        bytes
    }
}
