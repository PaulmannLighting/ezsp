use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0108;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    data_type: u8,
    data: ByteSizedVec<u8>,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    #[must_use]
    pub const fn data_ptr(&self) -> &ByteSizedVec<u8> {
        &self.data
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
