pub const ID: u16 = 0x0067;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    destination_node_id: EmberNodeId,
    destination_eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(destination_node_id: EmberNodeId, destination_eui64: EmberEUI64) -> Self {
        Self {
            destination_node_id,
            destination_eui64,
        }
    }

    #[must_use]
    pub const fn destination_node_id(&self) -> EmberNodeId {
        self.destination_node_id
    }

    #[must_use]
    pub const fn destination_eui64(&self) -> EmberEUI64 {
        self.destination_eui64
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
