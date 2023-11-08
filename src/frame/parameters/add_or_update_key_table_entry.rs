pub const ID: u16 = 0x0066;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    address: EmberEUI64,
    link_key: bool,
    key_data: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(address: EmberEUI64, link_key: bool, key_data: EmberKeyData) -> Self {
        Self {
            address,
            link_key,
            key_data,
        }
    }

    #[must_use]
    pub const fn address(&self) -> EmberEUI64 {
        self.address
    }

    #[must_use]
    pub const fn link_key(&self) -> bool {
        self.link_key
    }

    #[must_use]
    pub const fn key_data(&self) -> EmberKeyData {
        self.key_data
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
