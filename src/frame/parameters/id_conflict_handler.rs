pub const ID: u16 = 0x007C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(id: EmberNodeId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(&self) -> EmberNodeId {
        self.id
    }
}
