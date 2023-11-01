pub mod header;
pub mod legacy_parameters;
pub mod parameters;

use crate::frame::header::{Control, Header, LegacyHeader};
use crate::frame::legacy_parameters::LegacyParameters;
use crate::frame::parameters::Parameters;
use crate::read_write::{Readable, Writable};
use anyhow::anyhow;
use std::fmt::Debug;
use std::io::{Read, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    header: Header,
    parameters: Parameters,
}

impl Frame {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, parameters: Parameters) -> Self {
        Self {
            header: Header::new(sequence, control, parameters.id()),
            parameters,
        }
    }

    /// Returns the header
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Returns the payload
    #[must_use]
    pub const fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl Readable for Frame {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Header::read_from(src)?;
        let parameters = match header.id() {
            parameters::networking::network_init::ID => Parameters::Response(
                parameters::Response::Networking(parameters::networking::Response::NetworkInit(
                    parameters::networking::network_init::Response::read_from(src)?,
                )),
            ),
            n => return Err(anyhow!("Invalid header id: {n:#04X}")),
        };

        Ok(Self { header, parameters })
    }
}

impl Writable for Frame {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.header.write_to(dst)?;
        self.parameters.write_to(dst)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct LegacyFrame {
    header: LegacyHeader,
    parameters: LegacyParameters,
}

impl LegacyFrame {
    #[must_use]
    pub const fn new(sequence: u8, control: u8, parameters: LegacyParameters) -> Self {
        Self {
            header: LegacyHeader::new(sequence, control, parameters.id()),
            parameters,
        }
    }

    /// Returns the header
    #[must_use]
    pub const fn header(&self) -> &LegacyHeader {
        &self.header
    }

    /// Returns the parameters
    #[must_use]
    pub const fn parameters(&self) -> &LegacyParameters {
        &self.parameters
    }
}

impl Readable for LegacyFrame {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = LegacyHeader::read_from(src)?;

        match header.id() {
            parameters::configuration::version::ID => {
                parameters::configuration::version::Response::read_from(src).map(|parameters| {
                    Self {
                        header,
                        parameters: parameters.into(),
                    }
                })
            }
            n => Err(anyhow!("Invalid header id: {n:#04X}")),
        }
    }
}

impl Writable for LegacyFrame {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        self.header.write_to(dst)?;
        self.parameters.write_to(dst)
    }
}
