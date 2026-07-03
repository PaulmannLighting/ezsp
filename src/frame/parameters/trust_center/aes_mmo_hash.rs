//! Parameters for the [`TrustCenter::aes_mmo_hash`](crate::TrustCenter::aes_mmo_hash) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::aes::MmoHashContext;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x006F, { context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8> }, { status: u8, return_context: MmoHashContext });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub const fn new(context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8>) -> Self {
        Self {
            context,
            finalize,
            data,
        }
    }
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
