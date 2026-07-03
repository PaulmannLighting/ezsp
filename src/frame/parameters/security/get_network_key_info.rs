//! Parameters for the [`Security::get_network_key_info`](crate::Security::get_network_key_info) command.

use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::NetworkKeyInfo;

use crate::Error;

crate::frame::parameters::frame!(0x0116, {}, { status: u32, network_key_info: NetworkKeyInfo });

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
