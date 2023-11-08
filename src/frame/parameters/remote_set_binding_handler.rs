
pub const ID: u16 = 0x0031;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    entry: EmberBindingTableEntry,
    index: u8,
    policy_decision: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(entry: EmberBindingTableEntry, index: u8, policy_decision: EmberStatus) -> Self {
        Self { entry, index, policy_decision }
    }

    #[must_use]
    pub const fn entry(&self) -> EmberBindingTableEntry {
        self.entry
    }


    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn policy_decision(&self) -> EmberStatus {
        self.policy_decision
    }
}
