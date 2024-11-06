//! Parameters for the [`Networking::set_radio_ieee802154_cca_mode`](crate::Networking::set_radio_ieee802154_cca_mode) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::disambiguation::Disambiguation;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0095;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    cca_mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(cca_mode: u8) -> Self {
        Self { cca_mode }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
    const DISAMBIGUATION: Option<Disambiguation> = Some(Disambiguation::SetRadioIeee802154CcaMode);
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
    const DISAMBIGUATION: Option<Disambiguation> = Some(Disambiguation::SetRadioIeee802154CcaMode);
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
