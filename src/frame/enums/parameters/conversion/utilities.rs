use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::utilities;

impl TryFrom<Parameters> for utilities::custom_frame::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::CustomFrame(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::debug_write::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::DebugWrite(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::delay_test::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::DelayTest(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::echo::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::Echo(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_eui64::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetEui64(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_library_status::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetLibraryStatus(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_mfg_token::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetMfgToken(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_node_id::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetNodeId(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_phy_interface_count::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(
                utilities::Response::GetPhyInterfaceCount(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_random_number::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetRandomNumber(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_timer::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetTimer(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_token::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetToken(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_true_random_entropy_source::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(
                utilities::Response::GetTrueRandomEntropySource(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::get_xncp_info::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::GetXncpInfo(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::nop::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::Nop(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::read_and_clear_counters::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(
                utilities::Response::ReadAndClearCounters(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::read_counters::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::ReadCounters(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::set_mfg_token::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::SetMfgToken(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::set_timer::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::SetTimer(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for utilities::set_token::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Utilities(utilities::Response::SetToken(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}
