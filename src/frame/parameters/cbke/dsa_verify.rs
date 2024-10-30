//! Parameters for [`Cbke::dsa_verify`](crate::Cbke::dsa_verify) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{CertificateData, MessageDigest, SignatureData, Status};
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00A3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    digest: MessageDigest,
    signer_certificate: CertificateData,
    received_sig: SignatureData,
}

impl Command {
    #[must_use]
    pub const fn new(
        digest: MessageDigest,
        signer_certificate: CertificateData,
        received_sig: SignatureData,
    ) -> Self {
        Self {
            digest,
            signer_certificate,
            received_sig,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
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
