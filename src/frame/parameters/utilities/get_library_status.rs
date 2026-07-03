//! Parameters for the [`Utilities::get_library_status`](crate::Utilities::get_library_status) command.

use crate::ember::library::{Id, Status};

crate::frame::parameters::frame!(
    0x0001,
    { library_id: Id },
    impl {
        impl Command {
            /// Creates command parameters.
            #[must_use]
            pub const fn new(library_id: Id) -> Self {
                Self { library_id }
            }
        }
    },
    { status: Status },
    impl {
        impl From<Response> for Status {
            fn from(response: Response) -> Self {
                response.status
            }
        }
    }
);
