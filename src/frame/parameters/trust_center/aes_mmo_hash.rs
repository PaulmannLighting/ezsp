//! Parameters for the [`TrustCenter::aes_mmo_hash`](crate::TrustCenter::aes_mmo_hash) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::aes::MmoHashContext;
use crate::ember::Status;
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x006F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: MmoHashContext,
    finalize: bool,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8>) -> Self {
        Self {
            context,
            finalize,
            data,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    return_context: MmoHashContext,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
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
