use crate::frame::enums::{Parameters, Response};
use crate::frame::parameters::utilities;

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
