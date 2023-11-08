
pub const ID: u16 = 0x002C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
    value: EmberBindingTableEntry,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, value: EmberBindingTableEntry) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }


    #[must_use]
    pub const fn value(&self) -> EmberBindingTableEntry {
        self.value
    }
}
