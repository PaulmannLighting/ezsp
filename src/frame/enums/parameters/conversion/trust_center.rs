use crate::frame::enums::{Parameters, Response};
use crate::parameters::trust_center;

impl TryFrom<Parameters> for trust_center::aes_mmo_hash::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TrustCenter(trust_center::Response::AesMmoHash(
                response,
            ))) => Ok(*response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for trust_center::broadcast_network_key_switch::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TrustCenter(
                trust_center::Response::BroadcastNetworkKeySwitch(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for trust_center::broadcast_next_network_key::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TrustCenter(
                trust_center::Response::BroadcastNextNetworkKey(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for trust_center::remove_device::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TrustCenter(trust_center::Response::RemoveDevice(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for trust_center::unicast_nwk_key_update::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TrustCenter(
                trust_center::Response::UnicastNwkKeyUpdate(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}
