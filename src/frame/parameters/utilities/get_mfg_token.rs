//! Parameters for the [`Utilities::get_mfg_token`](crate::Utilities::get_mfg_token) command.

use crate::ezsp::mfg_token::Id;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(0x000B, { token_id: u8 }, { token_data: ByteSizedVec<u8> });

impl Command {
    /// Creates command parameters.
    #[must_use]
    pub fn new(token_id: Id) -> Self {
        Self {
            token_id: token_id.into(),
        }
    }
}

/// Convert the response into the token data.
impl From<Response> for ByteSizedVec<u8> {
    fn from(response: Response) -> Self {
        response.token_data
    }
}
