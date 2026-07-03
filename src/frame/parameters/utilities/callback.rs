//! Parameters for the [`Utilities::callback`](crate::Utilities::callback) command.

use crate::Parameters;
use crate::frame::RespondsWith;

crate::frame::parameters::command!(0x0006, {});

impl RespondsWith for Command {
    type Response = Parameters;
}
