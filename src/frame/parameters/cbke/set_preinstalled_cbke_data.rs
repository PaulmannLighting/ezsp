//! Parameters for the [`Cbke::set_preinstalled_cbke_data`](crate::Cbke::set_preinstalled_cbke_data) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{CertificateData, PrivateKeyData, PublicKeyData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x00A2;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    ca_public: PublicKeyData,
    my_cert: CertificateData,
    my_key: PrivateKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(
        ca_public: PublicKeyData,
        my_cert: CertificateData,
        my_key: PrivateKeyData,
    ) -> Self {
        Self {
            ca_public,
            my_cert,
            my_key,
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

/// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
