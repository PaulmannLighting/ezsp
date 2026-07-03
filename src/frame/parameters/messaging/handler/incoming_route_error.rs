use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{NodeId, Status};

crate::frame::parameters::handler!(
    0x0080,
    { status: u8, target: NodeId },
    impl {
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
                match Status::from_u8(self.status).ok_or(self.status) {
                    Ok(Status::SourceRouteFailure) => Ok(Status::SourceRouteFailure),
                    Ok(Status::ManyToOneRouteFailure) => Ok(Status::ManyToOneRouteFailure),
                    other => Err(other.into()),
                }
            }

            /// The short id of the remote node.
            #[must_use]
            pub const fn target(&self) -> NodeId {
                self.target
            }
        }
    }
);
