//! Parameters for the [`Cbke::get_certificate283k1`](crate::Cbke::get_certificate283k1) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Certificate283k1Data, Status};
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x00EC;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    local_cert: Certificate283k1Data,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into [`Certificate283k1Data`]
/// or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for Certificate283k1Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.local_cert),
            other => Err(other.into()),
        }
    }
}
