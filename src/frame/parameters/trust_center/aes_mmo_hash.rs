//! Parameters for the [`TrustCenter::aes_mmo_hash`](crate::TrustCenter::aes_mmo_hash) command.

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::aes::MmoHashContext;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x006F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: MmoHashContext,
    finalize: bool,
    data: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Command {
    #[must_use]
    pub fn new(context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8>) -> Self {
        Self {
            context,
            finalize,
            data: data.into(),
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    return_context: MmoHashContext,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`MmoHashContext`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for MmoHashContext {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.return_context),
            other => Err(other.into()),
        }
    }
}
