use crate::ember::Status;
use crate::read_write::Readable;
use crate::types::ByteVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};
use std::vec::IntoIter;

pub const ID: u16 = 0x0012;

/// Sends a debug message from the Host to the Network Analyzer utility via the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    binary_message: bool,
    message_contents: ByteVec,
}

impl Command {
    /// Creates new [`Command`] payload
    pub fn new(binary_message: bool, message_contents: ByteVec) -> Self {
        Self {
            binary_message,
            message_contents,
        }
    }

    #[must_use]
    pub const fn binary_message(&self) -> bool {
        self.binary_message
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_contents.len() as u8
    }

    #[must_use]
    pub fn message_contents(&self) -> &[u8] {
        &self.message_contents
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.message_contents.len());
        parameters.push(self.binary_message.into());
        parameters.push(self.message_length());
        parameters.extend_from_slice(&self.message_contents);
        parameters.into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let binary_message = src.read_bool()?;
        let message_length: u8 = src.read_num_le()?;
        Ok(Self {
            binary_message,
            message_contents: unsafe { src.read_heapless_vec_exact(message_length as usize)? },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
