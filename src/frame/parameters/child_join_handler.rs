pub const ID: u16 = 0x0023;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    index: u8,
    joining: bool,
    child_id: EmberNodeId,
    child_eui64: EmberEUI64,
    child_type: EmberNodeType,
}

impl Response {
    #[must_use]
    pub const fn new(
        index: u8,
        joining: bool,
        child_id: EmberNodeId,
        child_eui64: EmberEUI64,
        child_type: EmberNodeType,
    ) -> Self {
        Self {
            index,
            joining,
            child_id,
            child_eui64,
            child_type,
        }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn joining(&self) -> bool {
        self.joining
    }

    #[must_use]
    pub const fn child_id(&self) -> EmberNodeId {
        self.child_id
    }

    #[must_use]
    pub const fn child_eui64(&self) -> EmberEUI64 {
        self.child_eui64
    }

    #[must_use]
    pub const fn child_type(&self) -> EmberNodeType {
        self.child_type
    }
}
