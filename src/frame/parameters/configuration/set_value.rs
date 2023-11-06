use crate::ezsp::value::Id;
use crate::ezsp::Status;
use crate::read_write::Readable;
use crate::types::ByteVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};
use std::vec::IntoIter;

pub const ID: u16 = 0x00AB;

/// Writes a value to the NCP.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    value_id: Id,
    value: ByteVec,
}

impl Command {
    /// Crates new [`Command`] payload.
    pub fn new(value_id: Id, value: ByteVec) -> Self {
        Self { value_id, value }
    }

    #[must_use]
    pub const fn value_id(&self) -> Id {
        self.value_id
    }

    #[must_use]
    pub const fn value_length(&self) -> u8 {
        self.value.len() as u8
    }

    #[must_use]
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.value_id.into());
        parameters.push(self.value_length());
        parameters.extend_from_slice(&self.value);
        parameters.into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [value_id, value_length] = src.read_array_exact()?;
        Ok(Self {
            value_id: value_id.try_into()?,
            value: unsafe { src.read_heapless_vec_exact(value_length as usize)? },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
