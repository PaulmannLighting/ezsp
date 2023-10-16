use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x0012;

/// Sends a debug message from the Host to the Network Analyzer utility via the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    binary_message: bool,
    message_length: u8,
    message_contents: Arc<[u8]>,
}

impl Command {
    /// Creates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `message_contents` exceeds the bounds of an u8.
    pub fn new(
        sequence: u8,
        control: Control,
        binary_message: bool,
        message_contents: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.message_contents.len());
        parameters.push(self.binary_message.into());
        parameters.push(self.message_length);
        parameters.extend_from_slice(&self.message_contents);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [binary_message, message_length]: [u8; 2] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut message_contents = vec![0; message_length.into()];
        src.read_exact(&mut message_contents)?;
        Ok(Self {
            header,
            binary_message: binary_message != 0,
            message_length,
            message_contents: message_contents.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
        })
    }
}
