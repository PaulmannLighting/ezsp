use le_stream::derive::FromLeStream;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x0032;

/// The NCP used the external binding modification policy to decide
/// how to handle a remote delete binding request.
///
/// The Host cannot change the current decision, but it can change the policy
/// for future decisions using the setPolicy command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    index: u8,
    policy_decision: u8,
}

impl Handler {
    /// The index of the binding whose deletion was requested.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Returns the index of the binding whose deletion was requested
    /// if the binding was removed from the table and any other status if not.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the status is not [`Status::Success`].
    pub fn result(&self) -> Result<u8, Error> {
        Status::try_from(self.policy_decision)
            .resolve()
            .map(|()| self.index)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
