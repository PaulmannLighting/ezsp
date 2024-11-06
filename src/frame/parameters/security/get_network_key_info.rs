//! Parameters for the [`Security::get_network_key_info`](crate::Security::get_network_key_info) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::man::NetworkKeyInfo;
use siliconlabs::Status;

use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0116;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
    network_key_info: NetworkKeyInfo,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`NetworkKeyInfo`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for NetworkKeyInfo {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.network_key_info),
            other => Err(other.into()),
        }
    }
}
