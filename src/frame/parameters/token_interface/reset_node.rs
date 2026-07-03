//! Parameters for the [`TokenInterface::reset_node`](crate::TokenInterface::reset_node) command.

crate::frame::parameters::frame!(
    0x0104,
    {},
    {} => TokenInterface(token_interface)::ResetNode,
);
