use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

use crate::ember::binding::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::{Error, ValueError};

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

    /// `Ok(())` if the binding was added to the table.
    ///
    /// # Errors
    ///
    /// Returns an error if the status is not [`Status::Success`].
    pub fn policy_decision(&self) -> Result<(), Error> {
        Status::from_u8(self.policy_decision)
            .ok_or_else(|| ValueError::Ember(self.policy_decision).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(())
                } else {
                    Err(status.into())
                }
            })
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
