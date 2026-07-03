//! Parameters for the [`Configuration::send_pan_id_update`](crate::Configuration::send_pan_id_update) command.

use crate::ember::PanId;

crate::frame::parameters::frame!(0x0057, { new_pan: PanId }, { status: bool });

impl Command {
    #[must_use]
    pub const fn new(new_pan: PanId) -> Self {
        Self { new_pan }
    }
}

/// Converts the response into a [`bool`] indicating whether the command was successful.
impl From<Response> for bool {
    fn from(response: Response) -> Self {
        response.status
    }
}
