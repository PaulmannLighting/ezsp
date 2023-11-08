pub const ID: u16 = 0x00CA;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    key: EmberKeyData,
    security_type: SecureEzspSecurityType,
}

impl Command {
    #[must_use]
    pub const fn new(key: EmberKeyData, security_type: SecureEzspSecurityType) -> Self {
        Self { key, security_type }
    }

    #[must_use]
    pub const fn key(&self) -> EmberKeyData {
        self.key
    }

    #[must_use]
    pub const fn security_type(&self) -> SecureEzspSecurityType {
        self.security_type
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }
}
