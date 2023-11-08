
pub const ID: u16 = 0x00B3;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    network_key: EmberKeyData,
    security_state: EmberZllInitialSecurityState,
}

impl Command {
    #[must_use]
    pub const fn new(network_key: EmberKeyData, security_state: EmberZllInitialSecurityState) -> Self {
        Self { network_key, security_state }
    }

    #[must_use]
    pub const fn network_key(&self) -> EmberKeyData {
        self.network_key
    }


    #[must_use]
    pub const fn security_state(&self) -> EmberZllInitialSecurityState {
        self.security_state
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
