//! Parameters for the [`Networking::start_scan`](crate::Networking::start_scan) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::Status;

use crate::ezsp::network::scan::Type;
use crate::frame::Identified;
use crate::types::VariableLengthU32;
use crate::Error;

const ID: u16 = 0x001A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    scan_type: u8,
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub fn new(scan_type: Type, channel_mask: u32, duration: u8) -> Self {
        Self {
            scan_type: scan_type.into(),
            channel_mask,
            duration,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: VariableLengthU32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let status = response.status.into();
        match Status::from_u32(status).ok_or(status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
