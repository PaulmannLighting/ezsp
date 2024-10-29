use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::wwah;

impl TryFrom<Parameters> for wwah::get_parent_classification_enabled::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(
                wwah::Response::GetParentClassificationEnabled(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for wwah::is_hub_connected::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(wwah::Response::IsHubConnected(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for wwah::is_uptime_long::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(wwah::Response::IsUptimeLong(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for wwah::set_hub_connectivity::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(wwah::Response::SetHubConnectivity(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for wwah::set_long_uptime::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(wwah::Response::SetLongUptime(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for wwah::set_parent_classification_enabled::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Wwah(
                wwah::Response::SetParentClassificationEnabled(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}
