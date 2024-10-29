use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::binding;

impl TryFrom<Parameters> for binding::clear_table::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::ClearTable(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::delete::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::Delete(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::get::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::Get(response))) => {
                Ok(*response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::get_remote_node_id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::GetRemoteNodeId(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::is_active::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::IsActive(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::set::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::Set(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for binding::set_remote_node_id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Binding(binding::Response::SetRemoteNodeId(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
