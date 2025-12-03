use le_stream::FromLeStream;
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::binding::TableEntry;
use crate::frame::Parameter;

const ID: u16 = 0x0031;

/// The NCP used the external binding modification policy
/// to decide how to handle a remote set binding request.
///
/// The Host cannot change the current decision, but it can change the policy
/// for future decisions using the setPolicy command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    entry: TableEntry,
    index: u8,
    policy_decision: u8,
}

impl Handler {
    /// The requested binding.
    #[must_use]
    pub const fn entry(&self) -> &TableEntry {
        &self.entry
    }

    /// The index at which the binding was added.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Return the policy decision made by the NCP.
    ///
    /// # Errors
    ///
    /// Returns the [`u8`] value of the policy decision if it has an invalid value.
    pub fn policy_decision(&self) -> Result<Status, u8> {
        Status::from_u8(self.policy_decision).ok_or(self.policy_decision)
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}

impl TryFrom<Handler> for (u8, TableEntry) {
    type Error = Error;

    fn try_from(handler: Handler) -> Result<Self, Self::Error> {
        match handler.policy_decision() {
            Ok(Status::Success) => Ok((handler.index, handler.entry)),
            other => Err(other.into()),
        }
    }
}
