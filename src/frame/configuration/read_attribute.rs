use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x0108;

/// Read attribute data on NCP endpoints.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Command {
    type Parameters = [u8; 8];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let [cluster_low, cluster_high] = self.cluster.to_be_bytes();
        let [attribute_low, attribute_high] = self.attribute_id.to_be_bytes();
        let [manufacturer_code_low, manufacturer_code_high] = self.manufacturer_code.to_be_bytes();
        Some([
            self.endpoint,
            cluster_low,
            cluster_high,
            attribute_low,
            attribute_high,
            self.mask,
            manufacturer_code_low,
            manufacturer_code_high,
        ])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [
            endpoint,
            cluster_low,
            cluster_high,
            attribute_low,
            attribute_high,
            mask,
            manufacturer_code_low,
            manufacturer_code_high
        ]: [u8; 8] = [0; 8];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
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
    header: Header,
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
    pub fn new(
        sequence: u8,
        control: Control,
        status: Status,
        data_type: u8,
        data: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(3 + self.data.len());
        parameters.push(self.status.into());
        parameters.push(self.data_type);
        parameters.push(self.read_length);
        parameters.extend_from_slice(&self.data);
        Some(parameters)
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status, data_type, read_length]: [u8; 3] = [0; 3];
        src.read_exact(&mut buffer)?;
        let mut data = vec![0; read_length.into()];
        src.read_exact(&mut data)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
            data_type,
            read_length,
            data: data.into(),
        })
    }
}
