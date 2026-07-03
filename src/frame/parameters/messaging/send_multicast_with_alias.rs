//! Parameters for the [`Messaging::send_multicast_with_alias`](crate::Messaging::send_multicast_with_alias) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::aps::Frame;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(
    0x003A,
    { aps_frame: Frame, hops: u8, nonmember_radius: u8, alias: u16, nwk_sequence: u8, message_tag: u8, message_contents: ByteSizedVec<u8> },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(
                aps_frame: Frame,
                hops: u8,
                nonmember_radius: u8,
                alias: u16,
                nwk_sequence: u8,
                message_tag: u8,
                message_contents: ByteSizedVec<u8>,
            ) -> Self {
                Self {
                    aps_frame,
                    hops,
                    nonmember_radius,
                    alias,
                    nwk_sequence,
                    message_tag,
                    message_contents,
                }
            }
        }
    },
    { status: u8, sequence: u8 } => Messaging(messaging)::SendMulticastWithAlias,
    impl {
        /// Converts the response into the sequence number or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for u8 {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(response.sequence),
                    other => Err(other.into()),
                }
            }
        }
    }
);
