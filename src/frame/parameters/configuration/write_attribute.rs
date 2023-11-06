use crate::ember::Status;
use crate::read_write::Readable;
use crate::types::ByteSizedVec;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Once};
use std::vec::IntoIter;

pub const ID: u16 = 0x0109;

/// Write attribute data on NCP endpoints.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
    override_read_only_and_data_type: bool,
    just_test: bool,
    data_type: u8,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
        override_read_only_and_data_type: bool,
        just_test: bool,
        data_type: u8,
        data: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
            override_read_only_and_data_type,
            just_test,
            data_type,
            data,
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

    #[must_use]
    pub const fn override_read_only_and_data_type(&self) -> bool {
        self.override_read_only_and_data_type
    }

    #[must_use]
    pub const fn just_test(&self) -> bool {
        self.just_test
    }

    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn data_length(&self) -> u8 {
        self.data.len() as u8
    }

    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters = Vec::with_capacity(12 + self.data.len());
        parameters.push(self.endpoint);
        parameters.extend_from_slice(&self.cluster.to_le_bytes());
        parameters.extend_from_slice(&self.attribute_id.to_le_bytes());
        parameters.push(self.mask);
        parameters.extend_from_slice(&self.manufacturer_code.to_le_bytes());
        parameters.push(self.override_read_only_and_data_type.into());
        parameters.push(self.just_test.into());
        parameters.push(self.data_type);
        parameters.push(self.data_length());
        parameters.extend_from_slice(&self.data);
        parameters.into_iter()
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
        let override_read_only_and_data_type = src.read_bool()?;
        let just_test = src.read_bool()?;
        let [data_type, data_length] = src.read_array_exact()?;
        Ok(Self {
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
            override_read_only_and_data_type,
            just_test,
            data_type,
            data: unsafe { src.read_heapless_vec_exact(data_length as usize)? },
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
    pub const fn status(&self) -> Status {
        self.status
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
