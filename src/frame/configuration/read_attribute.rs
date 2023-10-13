use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;
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

    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    pub const fn cluster(&self) -> u16 {
        self.cluster
    }

    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    pub const fn mask(&self) -> u8 {
        self.mask
    }

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

    pub const fn status(&self) -> &Status {
        &self.status
    }

    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    pub const fn read_length(&self) -> u8 {
        self.read_length
    }

    pub const fn data(&self) -> &[u8] {
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
        parameters.push(self.status.to_u8().expect("could not convert status to u8"));
        parameters.push(self.data_type);
        parameters.push(self.read_length);
        parameters.extend_from_slice(&self.data);
        Some(parameters)
    }
}
