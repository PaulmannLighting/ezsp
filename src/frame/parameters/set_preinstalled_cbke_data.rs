use crate::ember::{CertificateData, PrivateKeyData, PublicKeyData, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00A2;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn ca_public(&self) -> PublicKeyData {
        self.ca_public
    }

    #[must_use]
    pub const fn my_cert(&self) -> CertificateData {
        self.my_cert
    }

    #[must_use]
    pub const fn my_key(&self) -> PrivateKeyData {
        self.my_key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
