//! Parameters for the [`Utilities::get_library_status`](crate::Utilities::get_library_status) command.

use crate::ember::library::{Id, Status};

crate::frame::parameters::frame!(0x0001, { library_id: Id }, { status: Status });

impl Command {
    #[must_use]
    pub const fn new(library_id: Id) -> Self {
        Self { library_id }
    }
}

impl From<Response> for Status {
    fn from(response: Response) -> Self {
        response.status
    }
}
