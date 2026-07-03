//! Parameters for the [`Utilities::read_counters`](crate::Utilities::read_counters) command.

use crate::ember::constants::COUNTER_TYPE_COUNT;

crate::frame::parameters::frame!(
    0x00F1,
    {},
    { values: [u16; COUNTER_TYPE_COUNT] },
    impl {
        /// Convert the response into an array of counter values.
        impl From<Response> for [u16; COUNTER_TYPE_COUNT] {
            fn from(response: Response) -> Self {
                response.values
            }
        }
    }
);
