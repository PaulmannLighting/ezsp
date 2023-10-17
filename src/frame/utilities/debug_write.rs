use crate::frame::Parameters;
use crate::status::Status;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;
use std::{array, vec};

pub const ID: u16 = 0x0012;

/// Sends a debug message from the Host to the Network Analyzer utility via the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    binary_message: bool,
    message_length: u8,
    message_contents: Arc<[u8]>,
}

impl Command {
    /// Creates new [`Command`] payload
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `message_contents` exceeds the bounds of an u8.
    pub fn new(binary_message: bool, message_contents: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            binary_message,
            message_length: message_contents.len().try_into()?,
            message_contents,
        })
    }

    #[must_use]
    pub const fn binary_message(&self) -> bool {
        self.binary_message
    }

    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }

    #[must_use]
    pub fn message_contents(&self) -> &[u8] {
        &self.message_contents
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.message_contents.len());
        parameters.push(self.binary_message.into());
        parameters.push(self.message_length);
        parameters.extend_from_slice(&self.message_contents);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [binary_message, message_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut message_contents = vec![0; message_length.into()];
        src.read_exact(&mut message_contents)?;
        Ok(Self {
            binary_message: binary_message != 0,
            message_length,
            message_contents: message_contents.into(),
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
    type IntoIter = array::IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
