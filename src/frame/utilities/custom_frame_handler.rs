use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use std::io::Read;
use std::sync::Arc;

const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    payload_length: u8,
    payload: Arc<[u8]>,
}

impl Response {
    /// Creates a new response
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if the payload exceeds its maximum size.
    pub fn new(sequence: u8, control: Control, payload: Arc<[u8]>) -> anyhow::Result<Self> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Response {
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
