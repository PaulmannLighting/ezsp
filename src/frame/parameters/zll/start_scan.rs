//! Parameters for the [`Zll::start_scan`](crate::Zll::start_scan) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::node::Type;

crate::frame::parameters::frame!(
    0x00B4,
    { channel_mask: u32, radio_power_for_scan: i8, node_type: u8 },
    impl {
        impl Command {
            /// Create a new command to start a scan..
            #[must_use]
            pub fn new(channel_mask: u32, radio_power_for_scan: i8, node_type: Type) -> Self {
                Self {
                    channel_mask,
                    radio_power_for_scan,
                    node_type: node_type.into(),
                }
            }
        }
    },
    { status: u8 } => Zll(zll)::StartScan,
    impl {
        /// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
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
