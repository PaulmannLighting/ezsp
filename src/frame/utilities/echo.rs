use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x0081;

/// Variable length data from the Host is echoed back by the NCP.
///
/// This command has no other effects and is designed
/// for testing the link between the Host and NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    data_length: u8,
    data: Arc<[u8]>,
}

impl Command {
    /// Crates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `data` exceeds the bounds of an u8.
    pub fn new(sequence: u8, control: Control, data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            data_length: data.len().try_into()?,
            data,
        })
    }

    #[must_use]
    pub const fn data_length(&self) -> u8 {
        self.data_length
    }

    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.data.len());
        parameters.push(self.data_length);
        parameters.extend_from_slice(&self.data);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [data_length]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut data = Vec::with_capacity(data_length.into());
        src.read_exact(&mut data)?;
        Ok(Self {
            header,
            data_length,
            data: data.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    echo_length: u8,
    echo: Arc<[u8]>,
}

impl Response {
    /// Crates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `echo` exceeds the bounds of an u8.
    pub fn new(sequence: u8, control: Control, echo: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            echo_length: echo.len().try_into()?,
            echo,
        })
    }

    #[must_use]
    pub const fn echo_length(&self) -> u8 {
        self.echo_length
    }

    #[must_use]
    pub fn echo(&self) -> &[u8] {
        &self.echo
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.echo.len());
        parameters.push(self.echo_length);
        parameters.extend_from_slice(&self.echo);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [echo_length]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut echo = Vec::with_capacity(echo_length.into());
        src.read_exact(&mut echo)?;
        Ok(Self {
            header,
            echo_length,
            echo: echo.into(),
        })
    }
}
