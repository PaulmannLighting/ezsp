use crate::ember::Status;
use crate::read_write::Readable;
use crate::types::ByteVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};
use std::num::TryFromIntError;
use std::{array, vec};

pub const ID: u16 = 0x0108;

/// Read attribute data on NCP endpoints.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Self {
        Self {
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
        }
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    #[must_use]
    pub const fn cluster(&self) -> u16 {
        self.cluster
    }

    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    #[must_use]
    pub const fn mask(&self) -> u8 {
        self.mask
    }

    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<
                Chain<Once<Self::Item>, array::IntoIter<Self::Item, 2>>,
                array::IntoIter<Self::Item, 2>,
            >,
            Once<Self::Item>,
        >,
        array::IntoIter<Self::Item, 2>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.endpoint)
            .chain(self.cluster.to_le_bytes())
            .chain(self.attribute_id.to_le_bytes())
            .chain(once(self.mask))
            .chain(self.manufacturer_code.to_le_bytes())
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let endpoint = src.read_num_le()?;
        let cluster = src.read_num_le()?;
        let attribute_id = src.read_num_le()?;
        let mask = src.read_num_le()?;
        let manufacturer_code = src.read_num_le()?;
        Ok(Self {
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    data_type: u8,
    read_length: u8,
    data: ByteVec,
}

impl Response {
    /// Crates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `data` exceeds the bounds of an u8.
    pub fn new(status: Status, data_type: u8, data: ByteVec) -> Self {
        Self {
            status,
            data_type,
            read_length: data.len() as u8,
            data,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    #[must_use]
    pub const fn read_length(&self) -> u8 {
        self.read_length
    }

    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(3 + self.data.len());
        parameters.push(self.status.into());
        parameters.push(self.data_type);
        parameters.push(self.read_length);
        parameters.extend_from_slice(&self.data);
        parameters.into_iter()
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [status, data_type, read_length] = src.read_array_exact()?;
        Ok(Self {
            status: status.try_into()?,
            data_type,
            read_length,
            data: unsafe { src.read_heapless_vec_exact(read_length as usize)? },
        })
    }
}
