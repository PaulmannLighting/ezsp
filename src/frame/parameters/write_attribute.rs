use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberStatus};

pub const ID: u16 = 0x0109;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
    just_test: bool,
    data_type: u8,
    data_length: u8,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(endpoint: u8, cluster: u16, attribute_id: u16, mask: u8, manufacturer_code: u16, just_test: bool, data_type: u8, data_length: u8, data: ByteSizedVec<u8>) -> Self {
        Self { endpoint, cluster, attribute_id, mask, manufacturer_code, just_test, data_type, data_length, data }
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
    pub const fn just_test(&self) -> bool {
        self.just_test
    }


    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }


    #[must_use]
    pub const fn data_length(&self) -> u8 {
        self.data_length
    }


    #[must_use]
    pub const fn data(&self) -> ByteSizedVec<u8> {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
