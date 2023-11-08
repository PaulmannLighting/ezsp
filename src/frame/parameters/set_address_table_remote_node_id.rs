pub const ID: u16 = 0x005D;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    address_table_index: u8,
    id: EmberNodeId,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, id: EmberNodeId) -> Self {
        Self {
            address_table_index,
            id,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn id(&self) -> EmberNodeId {
        self.id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;
