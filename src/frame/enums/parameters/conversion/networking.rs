use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::networking;

impl TryFrom<Parameters> for networking::start_scan::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Networking(networking::Response::StartScan(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
