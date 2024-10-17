use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};
use le_stream::derive::FromLeStream;
use num_traits::FromPrimitive;

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
        Status::from_u8(self.status).map_or_else(
            || Err(ValueError::Ember(self.status).into()),
            |status| match status {
                Status::SourceRouteFailure | Status::ManyToOneRouteFailure => Ok(status),
                _ => Err(Error::Ember(status)),
            },
        )
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
