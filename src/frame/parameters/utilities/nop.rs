//! Parameters for the [`Utilities::nop`](crate::Utilities::nop) command.

crate::frame::parameters::frame!(
    0x0005,
    {},
    {} => Utilities(utilities)::Nop,
);
