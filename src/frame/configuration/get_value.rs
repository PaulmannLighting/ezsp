use crate::ezsp::Status;
use crate::frame::Parameters;
use crate::value;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;
use std::{array, vec};

pub const ID: u16 = 0x00AA;

/// Reads a value from the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    value_id: value::Id,
}

impl Command {
    #[must_use]
    pub const fn new(value_id: value::Id) -> Self {
        Self { value_id }
    }

    #[must_use]
    pub const fn value_id(&self) -> value::Id {
        self.value_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = array::IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.value_id.into()].into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [value_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            value_id: value::Id::try_from(value_id)?,
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
    pub const fn status(&self) -> Status {
        self.status
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.status.into());
        parameters.push(self.value_length);
        parameters.extend_from_slice(&self.value);
        parameters.into_iter()
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
