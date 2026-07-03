//! Parameters for the [`Messaging::send_unicast`](crate::Messaging::send_unicast) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::aps::Frame;
use crate::ember::message::Destination;
use crate::ember::{NodeId, Status};
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x0034, { typ: u8, index_or_destination: NodeId, aps_frame: Frame, tag: u8, message: ByteSizedVec<u8> }, { status: u8, sequence: u8 });

impl Command {
    #[must_use]
    pub const fn new(
        destination: Destination,
        aps_frame: Frame,
        tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            typ: destination.discriminant(),
            index_or_destination: match destination {
                Destination::Direct(node_id) => node_id,
                Destination::ViaAddressTable(index) | Destination::ViaBinding(index) => index,
            },
            aps_frame,
            tag,
            message,
        }
    }
}

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
