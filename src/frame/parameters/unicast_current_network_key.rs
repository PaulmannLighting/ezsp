
pub const ID: u16 = 0x0050;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    target_short: EmberNodeId,
    target_long: EmberEUI64,
    parent_short_id: EmberNodeId,
}

impl Command {
    #[must_use]
    pub const fn new(target_short: EmberNodeId, target_long: EmberEUI64, parent_short_id: EmberNodeId) -> Self {
        Self { target_short, target_long, parent_short_id }
    }

    #[must_use]
    pub const fn target_short(&self) -> EmberNodeId {
        self.target_short
    }


    #[must_use]
    pub const fn target_long(&self) -> EmberEUI64 {
        self.target_long
    }


    #[must_use]
    pub const fn parent_short_id(&self) -> EmberNodeId {
        self.parent_short_id
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
