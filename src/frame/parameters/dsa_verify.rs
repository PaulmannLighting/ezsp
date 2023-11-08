
pub const ID: u16 = 0x00A3;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    digest: EmberMessageDigest,
    signer_certificate: EmberCertificateData,
    received_sig: EmberSignatureData,
}

impl Command {
    #[must_use]
    pub const fn new(digest: EmberMessageDigest, signer_certificate: EmberCertificateData, received_sig: EmberSignatureData) -> Self {
        Self { digest, signer_certificate, received_sig }
    }

    #[must_use]
    pub const fn digest(&self) -> EmberMessageDigest {
        self.digest
    }


    #[must_use]
    pub const fn signer_certificate(&self) -> EmberCertificateData {
        self.signer_certificate
    }


    #[must_use]
    pub const fn received_sig(&self) -> EmberSignatureData {
        self.received_sig
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
