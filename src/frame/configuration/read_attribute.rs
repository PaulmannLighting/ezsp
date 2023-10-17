use crate::frame::Parameters;
use crate::status::Status;
use std::io::Read;
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
    type IntoIter = array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        let [cluster_low, cluster_high] = self.cluster.to_be_bytes();
        let [attribute_id_low, attribute_id_high] = self.attribute_id.to_be_bytes();
        let [manufacturer_code_low, manufacturer_code_high] = self.manufacturer_code.to_be_bytes();
        [
            self.endpoint,
            cluster_low,
            cluster_high,
            attribute_id_low,
            attribute_id_high,
            self.mask,
            manufacturer_code_low,
            manufacturer_code_high,
        ]
        .into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @
        [endpoint, cluster_low, cluster_high, attribute_low, attribute_high, mask, manufacturer_code_low, manufacturer_code_high] =
            [0; 8];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            endpoint,
            cluster: u16::from_be_bytes([cluster_low, cluster_high]),
            attribute_id: u16::from_be_bytes([attribute_low, attribute_high]),
            mask,
            manufacturer_code: u16::from_be_bytes([manufacturer_code_low, manufacturer_code_high]),
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
        let mut buffer @ [status, data_type, read_length] = [0; 3];
        src.read_exact(&mut buffer)?;
        let mut data = vec![0; read_length.into()];
        src.read_exact(&mut data)?;
        Ok(Self {
            status: Status::try_from(status)?,
            data_type,
            read_length,
            data: data.into(),
        })
    }
}
