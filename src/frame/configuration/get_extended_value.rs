use crate::ezsp::Status;
use crate::frame::Parameters;
use crate::util::ReadExt;
use crate::value;
use std::io::Read;
use std::iter::{once, Chain, Once};
use std::num::TryFromIntError;
use std::sync::Arc;
use std::{array, vec};

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

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, array::IntoIter<Self::Item, 4>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.value_id.into()).chain(self.characteristics.to_be_bytes())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let value_id: u8 = src.read_num_be()?;
        let characteristics = src.read_num_be()?;
        Ok(Self {
            value_id: value_id.try_into()?,
            characteristics,
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

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut bytes = Vec::with_capacity(2 + self.value.len());
        bytes.push(self.status.into());
        bytes.push(self.value_length);
        bytes.extend_from_slice(&self.value);
        bytes.into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [status, value_length] = src.read_array_exact()?;
        let value = src.read_vec_exact(value_length)?;
        Ok(Self {
            status: status.try_into()?,
            value_length,
            value: value.into(),
        })
    }
}
