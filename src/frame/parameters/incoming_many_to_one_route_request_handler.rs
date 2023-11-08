pub const ID: u16 = 0x007D;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    source: EmberNodeId,
    long_id: EmberEUI64,
    cost: u8,
}

impl Response {
    #[must_use]
    pub const fn new(source: EmberNodeId, long_id: EmberEUI64, cost: u8) -> Self {
        Self {
            source,
            long_id,
            cost,
        }
    }

    #[must_use]
    pub const fn source(&self) -> EmberNodeId {
        self.source
    }

    #[must_use]
    pub const fn long_id(&self) -> EmberEUI64 {
        self.long_id
    }

    #[must_use]
    pub const fn cost(&self) -> u8 {
        self.cost
    }
}
