use le_stream::derive::FromLeStream;

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0080;

/// A callback invoked when a route error message is received.
///
/// The error indicates that a problem routing to or from the target node was encountered.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    target: NodeId,
}

impl Handler {
    /// Returns the status.
    ///
    /// # Returns
    ///
    /// One of:
    ///
    /// - [`Status::SourceRouteFailure`]
    /// - [`Status::ManyToOneRouteFailure`]
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the value is not a valid status.
    pub fn status(&self) -> Result<Status, Error> {
        match Status::try_from(self.status) {
            Ok(status) => match status {
                Status::SourceRouteFailure | Status::ManyToOneRouteFailure => Ok(status),
                _ => Err(Error::Ember(status)),
            },
            Err(status) => Err(ValueError::Ember(status).into()),
        }
    }

    /// The short id of the remote node.
    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
