use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x0109;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
    override_read_only_and_data_type: bool,
    just_test: bool,
    data_type: u8,
    data: Arc<[u8]>,
}

impl Command {
    pub const fn new(
        header: Header,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
        override_read_only_and_data_type: bool,
        just_test: bool,
        data_type: u8,
        data: Arc<[u8]>,
    ) -> Self {
        Self {
            header,
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

    pub const fn override_read_only_and_data_type(&self) -> bool {
        self.override_read_only_and_data_type
    }

    pub const fn just_test(&self) -> bool {
        self.just_test
    }

    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    pub const fn data_length(&self) -> u8 {
        self.data.len().try_into().expect("data length exceeded u8")
    }

    pub const fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(12 + self.data.len());
        parameters.push(self.endpoint);
        parameters.extend_from_slice(&self.cluster.to_be_bytes());
        parameters.extend_from_slice(&self.attribute_id.to_be_bytes());
        parameters.push(self.mask);
        parameters.extend_from_slice(&self.manufacturer_code.to_be_bytes());
        parameters.push(self.override_read_only_and_data_type.into());
        parameters.push(self.just_test.into());
        parameters.push(self.data_type);
        parameters.push(self.data_length());
        parameters.extend_from_slice(&self.data);
        Some(parameters)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(header: Header, status: Status) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.to_u8().expect("could not convert status to u8")])
    }
}
