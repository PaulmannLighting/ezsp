//! Parameters for the [`Security::send_trust_center_link_key`](crate::Security::send_trust_center_link_key) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, NodeId, Status};

crate::frame::parameters::frame!(
    0x0067,
    { destination_node_id: NodeId, destination_eui64: Eui64 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(destination_node_id: NodeId, destination_eui64: Eui64) -> Self {
                Self {
                    destination_node_id,
                    destination_eui64,
                }
            }
        }
    },
    { status: u8 } => Security(security)::SendTrustCenterLinkKey,
    impl {
        /// Convert the response into `()` or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }
    }
);
