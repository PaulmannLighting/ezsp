pub const ID: u16 = 0x00FD;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    child_count: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_count: u8) -> Self {
        Self { child_count }
    }

    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    node_type: EmberNodeType,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, node_type: EmberNodeType) -> Self {
        Self { status, node_type }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn node_type(&self) -> EmberNodeType {
        self.node_type
    }
}
