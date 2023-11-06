use crate::read_write::Readable;
use crate::types::ByteVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::vec::IntoIter;

pub const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    payload: ByteVec,
}

impl Response {
    /// Creates new response parameters
    pub fn new(payload: ByteVec) -> Self {
        Self { payload }
    }

    #[must_use]
    pub const fn payload_length(&self) -> u8 {
        self.payload.len() as u8
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
        parameters.push(self.payload_length());
        parameters.extend_from_slice(&self.payload);
        parameters.into_iter()
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let payload_length: u8 = src.read_num_le()?;
        Ok(Self {
            payload: unsafe { src.read_heapless_vec_exact(payload_length as usize)? },
        })
    }
}
