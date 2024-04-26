use le_stream::derive::FromLeBytes;

use crate::ember::binding::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;

const ID: u16 = 0x0031;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    entry: TableEntry,
    index: u8,
    policy_decision: u8,
}

impl Handler {
    #[must_use]
    pub const fn entry(&self) -> &TableEntry {
        &self.entry
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    pub fn policy_decision(&self) -> Result<Status, u8> {
        Status::try_from(self.policy_decision)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
