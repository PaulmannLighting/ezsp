pub const ID: u16 = 0x00A9;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    dest_short: EmberNodeId,
    dest_long: EmberEUI64,
    key: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: EmberNodeId, dest_long: EmberEUI64, key: EmberKeyData) -> Self {
        Self {
            dest_short,
            dest_long,
            key,
        }
    }

    #[must_use]
    pub const fn dest_short(&self) -> EmberNodeId {
        self.dest_short
    }

    #[must_use]
    pub const fn dest_long(&self) -> EmberEUI64 {
        self.dest_long
    }

    #[must_use]
    pub const fn key(&self) -> EmberKeyData {
        self.key
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
