use crate::frame::enums::{Parameters, Response};
use crate::parameters::token_interface;

impl TryFrom<Parameters> for token_interface::get_token_count::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::GetTokenCount(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::get_token_data::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::GetTokenData(response),
            )) => Ok(*response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::get_token_info::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::GetTokenInfo(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::gp_security_test_vectors::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::GpSecurityTestVectors(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::reset_node::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::ResetNode(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::set_token_data::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::SetTokenData(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for token_interface::token_factory_reset::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::TokenInterface(
                token_interface::Response::TokenFactoryReset(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}
