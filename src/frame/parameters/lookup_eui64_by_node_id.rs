
pub const ID: u16 = 0x0061;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    node_id: EmberNodeId,
}

impl Command {
    #[must_use]
    pub const fn new(node_id: EmberNodeId) -> Self {
        Self { node_id }
    }

    #[must_use]
    pub const fn node_id(&self) -> EmberNodeId {
        self.node_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    eui64: EmberEUI64,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, eui64: EmberEUI64) -> Self {
        Self { status, eui64 }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }
}
