pub const ID: u16 = 0x002B;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    index: u8,
    value: EmberBindingTableEntry,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, value: EmberBindingTableEntry) -> Self {
        Self { index, value }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn value(&self) -> EmberBindingTableEntry {
        self.value
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
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
