//! Parameters for the [`Networking::get_network_parameters`](crate::Networking::get_network_parameters) command.

use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::error::ValueError;

crate::frame::parameters::frame!(
    0x0028,
    {},
    { status: u8, node_type: u8, parameters: Parameters } => Networking(networking)::GetNetworkParameters,
    impl {
        impl Response {
            /// Returns the status of the response.
            ///
            /// # Errors
            ///
            /// Returns the raw status byte if it does not correspond to a known [`Status`].
            pub fn status(&self) -> Result<Status, u8> {
                Status::from_u8(self.status).ok_or(self.status)
            }

            /// Returns the node type of the response.
            ///
            /// # Errors
            ///
            /// Returns the raw node type byte if it does not correspond to a known [`Type`].
            pub fn node_type(&self) -> Result<Type, u8> {
                Type::from_u8(self.node_type).ok_or(self.node_type)
            }

            /// Returns the network parameters of the response.
            #[must_use]
            pub const fn parameters(&self) -> &Parameters {
                &self.parameters
            }

            /// Consumes the response and returns the network parameters.
            #[must_use]
            pub const fn into_parameters(self) -> Parameters {
                self.parameters
            }
        }
        /// Convert a response into a [`Type`] and [`Parameters`] tuple
        /// or an appropriate [`Error`] depending on its status.
        impl TryFrom<Response> for (Type, Parameters) {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, Self::Error> {
                match Status::from_u8(response.status).ok_or(response.status) {
                    Ok(Status::Success) => Type::from_u8(response.node_type)
                        .ok_or_else(|| ValueError::EmberNodeType(response.node_type).into())
                        .map(|node_type| (node_type, response.parameters)),
                    other => Err(other.into()),
                }
            }
        }
    }
);
