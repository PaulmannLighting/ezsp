use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::cbke;

impl TryFrom<Parameters> for cbke::calculate_smacs::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::CalculateSmacs(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::calculate_smacs283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::CalculateSmacs283k1(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::clear_temporary_data_maybe_store_link_key::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(
                cbke::Response::ClearTemporaryDataMaybeStoreLinkKey(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::clear_temporary_data_maybe_store_link_key283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(
                cbke::Response::ClearTemporaryDataMaybeStoreLinkKey283k1(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::dsa_sign::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::DsaSign(response))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::dsa_verify::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::DsaVerify(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::dsa_verify283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::DsaVerify283k1(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::generate_cbke_keys::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::GenerateCbkeKeys(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::generate_cbke_keys283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::GenerateCbkeKeys283k1(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::get_certificate::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::GetCertificate(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::get_certificate283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::GetCertificate283k1(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::save_preinstalled_cbke_data283k1::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(
                cbke::Response::SavePreinstalledCbkeData283k1(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for cbke::set_preinstalled_cbke_data::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::Cbke(cbke::Response::SetPreinstalledCbkeData(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}
