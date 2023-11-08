pub const ID: u16 = 0x00A5;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    local_cert: EmberCertificateData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, local_cert: EmberCertificateData) -> Self {
        Self { status, local_cert }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn local_cert(&self) -> EmberCertificateData {
        self.local_cert
    }
}
