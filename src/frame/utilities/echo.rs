use crate::frame::Parameters;
use crate::util::ReadExt;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;
use std::vec::IntoIter;

pub const ID: u16 = 0x0081;

/// Variable length data from the Host is echoed back by the NCP.
///
/// This command has no other effects and is designed
/// for testing the link between the Host and NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    data_length: u8,
    data: Arc<[u8]>,
}

impl Command {
    /// Crates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `data` exceeds the bounds of an u8.
    pub fn new(data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(1 + self.data.len());
        parameters.push(self.data_length);
        parameters.extend_from_slice(&self.data);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let data_length = src.read_u8()?;
        let data = src.read_vec_exact(data_length)?;
        Ok(Self {
            data_length,
            data: data.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    echo_length: u8,
    echo: Arc<[u8]>,
}

impl Response {
    /// Crates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `echo` exceeds the bounds of an u8.
    pub fn new(echo: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(1 + self.echo.len());
        parameters.push(self.echo_length);
        parameters.extend_from_slice(&self.echo);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let echo_length = src.read_u8()?;
        let echo = src.read_vec_exact(echo_length)?;
        Ok(Self {
            echo_length,
            echo: echo.into(),
        })
    }
}
