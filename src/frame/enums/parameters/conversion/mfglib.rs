use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::mfglib;

impl TryFrom<Parameters> for mfglib::end::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::End(response))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::get_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::GetChannel(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::get_power::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::GetPower(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::send_packet::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::SendPacket(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::set_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::SetChannel(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::set_power::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::SetPower(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::start::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::Start(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::start_stream::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::StartStream(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::start_tone::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::StartTone(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::stop_stream::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::StopStream(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for mfglib::stop_tone::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::MfgLib(mfglib::Response::StopTone(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}
