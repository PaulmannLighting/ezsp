use crate::frame::Parameters;
use crate::status::Status;
use crate::value;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

pub const ID: u16 = 0x003;

/// Reads a value from the NCP but passes an extra argument specific to the value being retrieved.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    value_id: value::ExtendedId,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: value::ExtendedId, characteristics: u32) -> Self {
        Self {
            value_id,
            characteristics,
        }
    }

    #[must_use]
    pub const fn value_id(&self) -> value::ExtendedId {
        self.value_id
    }

    #[must_use]
    pub const fn characteristics(&self) -> u32 {
        self.characteristics
    }
}

impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        let mut bytes = Vec::with_capacity(5);
        bytes.push(command.value_id.into());
        bytes.extend_from_slice(&command.characteristics.to_be_bytes());
        bytes
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [value_id, characteristics @ ..] = [0; 5];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            value_id: value::ExtendedId::try_from(value_id)?,
            characteristics: u32::from_be_bytes(characteristics),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    value_length: u8,
    value: Arc<[u8]>,
}

impl Response {
    /// Creates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `value` exceeds the bounds of an u8.
    pub fn new(status: Status, value: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            status,
            value_length: value.len().try_into()?,
            value,
        })
    }

    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
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

impl From<Response> for Vec<u8> {
    fn from(response: Response) -> Self {
        let mut bytes = Vec::with_capacity(2 + response.value.len());
        bytes.push(response.status.into());
        bytes.push(response.value_length);
        bytes.extend_from_slice(&response.value);
        bytes
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status, value_length] = [0; 2];
        src.read_exact(&mut buffer)?;
        let mut value = vec![0; value_length.into()];
        src.read_exact(&mut value)?;
        Ok(Self {
            status: Status::try_from(status)?,
            value_length,
            value: value.into(),
        })
    }
}
