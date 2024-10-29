mod proxy_table;
mod sink_table;

use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::green_power;

impl TryFrom<Parameters> for green_power::send::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::Send(response))) => {
                Ok(response)
            }
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::sink_commission::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(green_power::Response::SinkCommission(
                response,
            ))) => Ok(response),
            _ => Err(parameters),
        }
    }
}

impl TryFrom<Parameters> for green_power::translation_table_clear::Response {
    type Error = Parameters;

    fn try_from(parameters: Parameters) -> Result<Self, Self::Error> {
        match parameters {
            Parameters::Response(Response::GreenPower(
                green_power::Response::TranslationTableClear(response),
            )) => Ok(response),
            _ => Err(parameters),
        }
    }
}
