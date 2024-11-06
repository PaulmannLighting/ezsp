//! Parameters for the [`Configuration::read_attribute`](crate::Configuration::read_attribute) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0108;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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
    const ID: u16 = ID;
}

/// Read attribute data.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Attribute {
    data_type: u8,
    data: ByteSizedVec<u8>,
}

impl Attribute {
    /// Attribute data type.
    #[must_use]
    pub const fn data_type(&self) -> u8 {
        self.data_type
    }

    /// Attribute data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    payload: Attribute,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into an [`Attribute`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Attribute {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.payload),
            other => Err(other.into()),
        }
    }
}
