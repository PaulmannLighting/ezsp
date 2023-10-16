use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use anyhow::anyhow;
use std::io::Read;
use std::sync::Arc;

const ID: u16 = 0x0047;
const MAX_PAYLOAD_SIZE: u8 = 119;

/// Provides the customer a custom EZSP frame.
///
/// On the NCP, these frames are only handled if the XNCP library is included.
/// On the NCP side these frames are handled in the emberXNcpIncomingCustomEzspMessageCallback() callback function.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    payload_length: u8,
    payload: Arc<[u8]>,
}

impl Command {
    /// Creates a new command
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the payload exceeds [`MAX_PAYLOAD_SIZE`].
    pub fn new(sequence: u8, control: Control, payload: Arc<[u8]>) -> anyhow::Result<Self> {
        let payload_length: u8 = payload.len().try_into()?;

        if payload_length > MAX_PAYLOAD_SIZE {
            return Err(anyhow!(
                "payload size exceeded: {payload_length} > {MAX_PAYLOAD_SIZE}"
            ));
        }

        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            payload_length,
            payload,
        })
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload_length
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.payload.len());
        parameters.push(self.payload_length);
        parameters.extend_from_slice(&self.payload);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [payload_length] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut payload = vec![0; payload_length.into()];
        src.read_exact(&mut payload)?;
        Ok(Self {
            header,
            payload_length,
            payload: payload.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
    reply_length: u8,
    reply: Arc<[u8]>,
}

impl Response {
    /// Creates a new response
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the reply is too long.
    pub fn new(
        sequence: u8,
        control: Control,
        status: Status,
        reply: Arc<[u8]>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
            reply_length: reply.len().try_into()?,
            reply,
        })
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn reply_length(&self) -> u8 {
        self.reply_length
    }

    #[must_use]
    pub fn reply(&self) -> &[u8] {
        &self.reply
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.reply.len());
        parameters.push(self.status.into());
        parameters.push(self.reply_length);
        parameters.extend_from_slice(&self.reply);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status, reply_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut reply = vec![0; reply_length.into()];
        src.read_exact(&mut reply)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
            reply_length,
            reply: reply.into(),
        })
    }
}
