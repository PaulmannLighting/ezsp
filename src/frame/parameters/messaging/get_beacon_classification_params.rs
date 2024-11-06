//! Parameters for the [`Messaging::get_beacon_classification_params`](crate::Messaging::get_beacon_classification_params) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::beacon::ClassificationParams;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00F3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    param: ClassificationParams,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into the [`ClassificationParams`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for ClassificationParams {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.param),
            other => Err(other.into()),
        }
    }
}
