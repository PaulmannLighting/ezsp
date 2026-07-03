//! Parameters for the [`Utilities::get_true_random_entropy_source`](crate::Utilities::get_true_random_entropy_source) command.

use num_traits::FromPrimitive;

use crate::ember::entropy::Source;
use crate::{Error, ValueError};

crate::frame::parameters::frame!(
    0x004F,
    {},
    { entropy_source: u8 } => Utilities(utilities)::GetTrueRandomEntropySource,
    impl {
        /// Convert the response into a [`Source`] or an appropriate [`Error`]
        /// depending on the validity of its entropy source data.
        impl TryFrom<Response> for Source {
            type Error = Error;

            fn try_from(response: Response) -> Result<Self, <Self as TryFrom<Response>>::Error> {
                Self::from_u8(response.entropy_source)
                    .ok_or_else(|| ValueError::EntropySource(response.entropy_source).into())
            }
        }
    }
);
