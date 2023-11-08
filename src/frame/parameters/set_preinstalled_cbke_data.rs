
pub const ID: u16 = 0x00A2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    ca_public: EmberPublicKeyData,
    my_cert: EmberCertificateData,
    my_key: EmberPrivateKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(ca_public: EmberPublicKeyData, my_cert: EmberCertificateData, my_key: EmberPrivateKeyData) -> Self {
        Self { ca_public, my_cert, my_key }
    }

    #[must_use]
    pub const fn ca_public(&self) -> EmberPublicKeyData {
        self.ca_public
    }


    #[must_use]
    pub const fn my_cert(&self) -> EmberCertificateData {
        self.my_cert
    }


    #[must_use]
    pub const fn my_key(&self) -> EmberPrivateKeyData {
        self.my_key
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
