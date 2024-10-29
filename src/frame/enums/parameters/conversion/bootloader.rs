use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::bootloader;

impl TryFrom<Parameters> for bootloader::aes_encrypt::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Bootloader(bootloader::Response::AesEncrypt(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters>
    for bootloader::get_standalone_bootloader_version_plat_micro_phy::Response
{
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Bootloader(
                bootloader::Response::GetStandaloneBootloaderVersionPlatMicroPhy(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for bootloader::launch_standalone_bootloader::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Bootloader(
                bootloader::Response::LaunchStandaloneBootloader(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for bootloader::override_current_channel::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Bootloader(
                bootloader::Response::OverrideCurrentChannel(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for bootloader::send_bootload_message::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Bootloader(
                bootloader::Response::SendBootloadMessage(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}
