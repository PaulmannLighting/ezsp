use crate::read_write::Readable;
use crate::types::ByteSizedVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::vec::IntoIter;

pub const ID: u16 = 0x0081;

/// Variable length data from the Host is echoed back by the NCP.
///
/// This command has no other effects and is designed
/// for testing the link between the Host and NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(data: ByteSizedVec<u8>) -> Self {
        Self { data }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn data_length(&self) -> u8 {
        self.data.len() as u8
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
        parameters.push(self.data_length());
        parameters.extend_from_slice(&self.data);
        parameters.into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let data_length: u8 = src.read_num_le()?;
        Ok(Self {
            data: unsafe { src.read_heapless_vec_exact(data_length as usize)? },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    echo: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(echo: ByteSizedVec<u8>) -> Self {
        Self { echo }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn echo_length(&self) -> u8 {
        self.echo.len() as u8
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
        parameters.push(self.echo_length());
        parameters.extend_from_slice(&self.echo);
        parameters.into_iter()
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let echo_length: u8 = src.read_num_le()?;
        Ok(Self {
            echo: unsafe { src.read_heapless_vec_exact(echo_length as usize)? },
        })
    }
}
