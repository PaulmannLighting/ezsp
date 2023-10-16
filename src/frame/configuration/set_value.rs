use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use crate::value;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x00AB;

/// Writes a value to the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    value_id: value::Id,
    value_length: u8,
    value: Arc<[u8]>,
}

impl Command {
    /// Crates a new [`Command`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `value` exceeds the bounds of an u8.
    pub fn new(
        sequence: u8,
        control: Control,
        value_id: value::Id,
        value: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            value_id,
            value_length: value.len().try_into()?,
            value,
        })
    }

    #[must_use]
    pub const fn value_id(&self) -> &value::Id {
        &self.value_id
    }

    #[must_use]
    pub const fn value_length(&self) -> u8 {
        self.value_length
    }

    #[must_use]
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.value_id.into());
        parameters.push(self.value_length);
        parameters.extend_from_slice(&self.value);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [value_id, value_length]: [u8; 2] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut value = vec![0; value_length.into()];
        src.read_exact(&mut value)?;
        Ok(Self {
            header,
            value_id: value::Id::try_from(value_id)?,
            value_length,
            value: value.into(),
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
    pub const fn status(&self) -> &Status {
        &self.status
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
