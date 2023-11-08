
pub const ID: u16 = 0x0107;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    child_id: EmberNodeId,
}

impl Command {
    #[must_use]
    pub const fn new(child_id: EmberNodeId) -> Self {
        Self { child_id }
    }

    #[must_use]
    pub const fn child_id(&self) -> EmberNodeId {
        self.child_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    child_index: u8,
}

impl Response {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }

    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}
