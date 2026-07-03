//! Parameters for the [`Configuration::set_configuration_value`](crate::Configuration::set_configuration_value) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::ezsp::config::Id;

crate::frame::parameters::frame!(
    0x0053,
    { config_id: u8, value: u16 },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub fn new(config_id: Id, value: u16) -> Self {
                Self {
                    config_id: config_id.into(),
                    value,
                }
            }
        }
    },
    { status: u8 } => Configuration(configuration)::SetConfigurationValue,
    impl {
        /// Converts the response into `()` or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for () {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Ok(()),
                    other => Err(other.into()),
                }
            }
        }
    }
);
