use crate::ezsp_status::EzspStatus;
use crate::frame::Parameters;
use anyhow::anyhow;
use std::io::Read;
use std::sync::Arc;
use std::vec::IntoIter;

pub const ID: u16 = 0x0047;
const MAX_PAYLOAD_SIZE: u8 = 119;

/// Provides the customer a custom EZSP frame.
///
/// On the NCP, these frames are only handled if the XNCP library is included.
/// On the NCP side these frames are handled in the
/// emberXNcpIncomingCustomEzspMessageCallback() callback function.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    payload_length: u8,
    payload: Arc<[u8]>,
}

impl Command {
    /// Creates new command payload
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the payload exceeds [`MAX_PAYLOAD_SIZE`].
    pub fn new(payload: Arc<[u8]>) -> anyhow::Result<Self> {
        let payload_length: u8 = payload.len().try_into()?;

        if payload_length > MAX_PAYLOAD_SIZE {
            return Err(anyhow!(
                "payload size exceeded: {payload_length} > {MAX_PAYLOAD_SIZE}"
            ));
        }

        Ok(Self {
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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(1 + self.payload.len());
        parameters.push(self.payload_length);
        parameters.extend_from_slice(&self.payload);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [payload_length] = [0; 1];
        src.read_exact(&mut buffer)?;
        let mut payload = vec![0; payload_length.into()];
        src.read_exact(&mut payload)?;
        Ok(Self {
            payload_length,
            payload: payload.into(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: EzspStatus,
    reply_length: u8,
    reply: Arc<[u8]>,
}

impl Response {
    /// Creates a new response
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the reply is too long.
    pub fn new(status: EzspStatus, reply: Arc<[u8]>) -> anyhow::Result<Self> {
        Ok(Self {
            status,
            reply_length: reply.len().try_into()?,
            reply,
        })
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.reply.len());
        parameters.push(self.status.into());
        parameters.push(self.reply_length);
        parameters.extend_from_slice(&self.reply);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status, reply_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut reply = vec![0; reply_length.into()];
        src.read_exact(&mut reply)?;
        Ok(Self {
            status: EzspStatus::try_from(status)?,
            reply_length,
            reply: reply.into(),
        })
    }
}
