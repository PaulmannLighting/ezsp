//! Token Interface Frames

pub mod get_token_count;
pub mod get_token_data;
pub mod get_token_info;
pub mod gp_security_test_vectors;
pub mod reset_node;
pub mod set_token_data;
pub mod token_factory_reset;

/// Token Interface response parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Response parameter of the `get_token_count` command.
    GetTokenCount(get_token_count::Response),
    /// Response parameter of the `get_token_data` command.
    GetTokenData(Box<get_token_data::Response>),
    /// Response parameter of the `get_token_info` command.
    GetTokenInfo(get_token_info::Response),
    /// Response parameter of the `gp_security_test_vectors` command.
    GpSecurityTestVectors(gp_security_test_vectors::Response),
    /// Response parameter of the `reset_node` command.
    ResetNode(reset_node::Response),
    /// Response parameter of the `set_token_data` command.
    SetTokenData(set_token_data::Response),
    /// Response parameter of the `token_factory_reset` command.
    TokenFactoryReset(token_factory_reset::Response),
}
