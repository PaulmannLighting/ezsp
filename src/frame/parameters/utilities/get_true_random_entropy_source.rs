//! Parameters for the [`Utilities::get_true_random_entropy_source`](crate::Utilities::get_true_random_entropy_source) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::entropy::Source;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x004F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    entropy_source: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into a [`Source`] or an appropriate [`Error`]
/// depending on the validity of its entropy source data.
impl TryFrom<Response> for Source {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, <Self as TryFrom<Response>>::Error> {
        Self::from_u8(response.entropy_source)
            .ok_or_else(|| ValueError::EntropySource(response.entropy_source).into())
    }
}
