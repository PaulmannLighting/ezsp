use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{CertificateData, PrivateKeyData, PublicKeyData, Status};
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x00A2;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
