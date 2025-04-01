//! Parameters for the [`Configuration::write_attribute`](crate::Configuration::write_attribute) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

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
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
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
