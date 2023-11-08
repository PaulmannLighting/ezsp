
pub const ID: u16 = 0x0027;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    eui64: EmberEUI64,
    node_id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(eui64: EmberEUI64, node_id: EmberNodeId) -> Self {
        Self { eui64, node_id }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }


    #[must_use]
    pub const fn node_id(&self) -> EmberNodeId {
        self.node_id
    }
}
