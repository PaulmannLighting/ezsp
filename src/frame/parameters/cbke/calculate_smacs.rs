use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{CertificateData, PublicKeyData, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x009F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    am_initiator: bool,
    partner_certificate: CertificateData,
    partner_ephemeral_public_key: PublicKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: CertificateData,
        partner_ephemeral_public_key: PublicKeyData,
    ) -> Self {
        Self {
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(())
                } else {
                    Err(status.into())
                }
            })
    }
}
