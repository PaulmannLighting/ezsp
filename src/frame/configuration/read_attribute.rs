use crate::ember::Status;
use crate::frame::Parameters;
use crate::util::ReadExt;
use std::io::Read;
use std::iter::{once, Chain, Once};
use std::num::TryFromIntError;
use std::sync::Arc;
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
            .chain(self.cluster.to_be_bytes())
            .chain(self.attribute_id.to_be_bytes())
            .chain(once(self.mask))
            .chain(self.manufacturer_code.to_be_bytes())
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let endpoint = src.read_u8()?;
        let cluster = src.read_u16_be()?;
        let attribute_id = src.read_u16_be()?;
        let mask = src.read_u8()?;
        let manufacturer_code = src.read_u16_be()?;
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
    data: Arc<[u8]>,
}

impl Response {
    /// Crates a new [`Response`]
    ///
    /// # Errors
    /// Returns an [`TryFromIntError`] if the size of `data` exceeds the bounds of an u8.
    pub fn new(status: Status, data_type: u8, data: Arc<[u8]>) -> Result<Self, TryFromIntError> {
        Ok(Self {
            status,
            data_type,
            read_length: data.len().try_into()?,
            data,
        })
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [status, data_type, read_length] = src.read_array_exact()?;
        let data = src.read_vec_exact(read_length)?;
        Ok(Self {
            status: status.try_into()?,
            data_type,
            read_length,
            data: data.into(),
        })
    }
}
