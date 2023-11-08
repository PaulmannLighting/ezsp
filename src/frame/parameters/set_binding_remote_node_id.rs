
pub const ID: u16 = 0x0030;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    index: u8,
    node_id: EmberNodeId,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, node_id: EmberNodeId) -> Self {
        Self { index, node_id }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn node_id(&self) -> EmberNodeId {
        self.node_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

