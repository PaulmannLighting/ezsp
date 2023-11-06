use crate::ember::Status;
use crate::read_write::Readable;
use crate::types::ByteSizedVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::vec::IntoIter;

pub const ID: u16 = 0x0047;
pub const MAX_PAYLOAD_SIZE: u8 = 119;

/// Provides the customer a custom EZSP frame.
///
/// On the NCP, these frames are only handled if the XNCP library is included.
/// On the NCP side these frames are handled in the
/// emberXNcpIncomingCustomEzspMessageCallback() callback function.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn payload_length(&self) -> u8 {
        self.payload.len() as u8
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
        parameters.push(self.payload_length());
        parameters.extend_from_slice(&self.payload);
        parameters.into_iter()
    }
}

impl Readable for Command {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let payload_length: u8 = src.read_num_le()?;
        Ok(Self {
            payload: unsafe { src.read_heapless_vec_exact(payload_length.into())? },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    reply: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, reply: ByteSizedVec<u8>) -> Self {
        Self { status, reply }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn reply_length(&self) -> u8 {
        self.reply.len() as u8
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
        parameters.push(self.reply_length());
        parameters.extend_from_slice(&self.reply);
        parameters.into_iter()
    }
}

impl Readable for Response {
    fn try_read<R>(src: &mut R) -> Result<Self, crate::Error>
    where
        R: Read,
    {
        let [status, reply_length] = src.read_array_exact()?;
        Ok(Self {
            status: status.try_into()?,
            reply: unsafe { src.read_heapless_vec_exact(reply_length as usize)? },
        })
    }
}
