//! Parameters for the [`Utilities::callback`](crate::Utilities::callback) command.

use crate::Parameters;

crate::frame::parameters::command!(
    0x0006,
    {},
    Parameters => Utilities(utilities)::Callback
);
