//! Parameters for the [`TrustCenter::remove_device`](crate::TrustCenter::remove_device) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00A8;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    dest_short: NodeId,
    dest_long: Eui64,
    target_long: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, target_long: Eui64) -> Self {
        Self {
            dest_short,
            dest_long,
            target_long,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

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
