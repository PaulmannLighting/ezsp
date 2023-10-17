use crate::frame::Parameters;
use std::io::Read;
use std::sync::Arc;
use std::vec::IntoIter;

pub const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    payload_length: u8,
    payload: Arc<[u8]>,
}

impl Response {
    /// Creates new response parameters
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the payload exceeds its maximum size.
    pub fn new(payload: Arc<[u8]>) -> anyhow::Result<Self> {
        Ok(Self {
            payload_length: payload.len().try_into()?,
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(1 + self.payload.len());
        parameters.push(self.payload_length);
        parameters.extend_from_slice(&self.payload);
        parameters.into_iter()
    }
}

impl Parameters<u16> for Response {
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
