//! Parameters for the [`Utilities::read_counters`](crate::Utilities::read_counters) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::constants::COUNTER_TYPE_COUNT;
use crate::frame::Parameter;

const ID: u16 = 0x0065;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    values: [u16; COUNTER_TYPE_COUNT],
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into an array of counter values.
impl From<Response> for [u16; COUNTER_TYPE_COUNT] {
    fn from(response: Response) -> Self {
        response.values
    }
}
