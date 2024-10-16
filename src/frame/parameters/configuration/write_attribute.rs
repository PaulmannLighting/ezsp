//! Write attribute data on NCP endpoints.

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0109;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    endpoint: u8,
    cluster: u16,
    attribute_id: u16,
    mask: u8,
    manufacturer_code: u16,
    just_test: bool,
    data_type: u8,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(endpoint: u8, cluster: u16, attribute: &Attribute, just_test: bool) -> Self {
        Self {
            endpoint,
            cluster,
            attribute_id: attribute.id,
            mask: attribute.mask,
            manufacturer_code: attribute.manufacturer_code,
            just_test,
            data_type: attribute.data_type,
            data: attribute.data.clone(),
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}

/// An attribute to write.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Attribute {
    id: u16,
    mask: u8,
    manufacturer_code: u16,
    data_type: u8,
    data: ByteSizedVec<u8>,
}

impl Attribute {
    /// Creates a new attribute.
    #[must_use]
    pub const fn new(
        id: u16,
        mask: u8,
        manufacturer_code: u16,
        data_type: u8,
        data: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            id,
            mask,
            manufacturer_code,
            data_type,
            data,
        }
    }
}
