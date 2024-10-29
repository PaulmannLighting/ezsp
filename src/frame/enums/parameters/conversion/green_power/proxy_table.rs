use crate::frame::enums::{Parameters, Response};
use crate::parameters::green_power;

impl TryFrom<Parameters> for green_power::proxy_table::get_entry::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::ProxyTable(
                green_power::proxy_table::Response::GetEntry(response),
            ))) => Ok(*response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::proxy_table::lookup::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::ProxyTable(
                green_power::proxy_table::Response::Lookup(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::proxy_table::process_gp_pairing::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::ProxyTable(
                green_power::proxy_table::Response::ProcessGpPairing(response),
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
