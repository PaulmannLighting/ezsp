pub const ID: u16 = 0x00CD;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EzspStatus,
    return_security_type: SecureEzspSecurityType,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus, return_security_type: SecureEzspSecurityType) -> Self {
        Self {
            status,
            return_security_type,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }

    #[must_use]
    pub const fn return_security_type(&self) -> SecureEzspSecurityType {
        self.return_security_type
    }
}
