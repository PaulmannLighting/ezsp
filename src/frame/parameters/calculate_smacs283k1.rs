pub const ID: u16 = 0x00EA;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    am_initiator: bool,
    partner_certificate: EmberCertificate283k1Data,
    partner_ephemeral_public_key: EmberPublicKey283k1Data,
}

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: EmberCertificate283k1Data,
        partner_ephemeral_public_key: EmberPublicKey283k1Data,
    ) -> Self {
        Self {
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        }
    }

    #[must_use]
    pub const fn am_initiator(&self) -> bool {
        self.am_initiator
    }

    #[must_use]
    pub const fn partner_certificate(&self) -> EmberCertificate283k1Data {
        self.partner_certificate
    }

    #[must_use]
    pub const fn partner_ephemeral_public_key(&self) -> EmberPublicKey283k1Data {
        self.partner_ephemeral_public_key
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
