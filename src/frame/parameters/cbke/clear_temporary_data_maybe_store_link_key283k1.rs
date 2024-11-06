//! Parameters for the [`Cbke::clear_temporary_data_maybe_store_link_key`](crate::Cbke::clear_temporary_data_maybe_store_link_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x00EE;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    store_link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(store_link_key: bool) -> Self {
        Self { store_link_key }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
