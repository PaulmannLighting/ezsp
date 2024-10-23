//! Token Interface Frames

pub mod get_token_count;
pub mod get_token_data;
pub mod get_token_info;
pub mod gp_security_test_vectors;
pub mod reset_node;
pub mod set_token_data;
pub mod token_factory_reset;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    GetTokenCount(get_token_count::Command),
    GetTokenData(get_token_data::Command),
    GetTokenInfo(get_token_info::Command),
    GpSecurityTestVectors(gp_security_test_vectors::Command),
    ResetNode(reset_node::Command),
    SetTokenData(set_token_data::Command),
    TokenFactoryReset(token_factory_reset::Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    GetTokenCount(get_token_count::Response),
    GetTokenData(get_token_data::Response),
    GetTokenInfo(get_token_info::Response),
    GpSecurityTestVectors(gp_security_test_vectors::Response),
    ResetNode(reset_node::Response),
    SetTokenData(set_token_data::Response),
    TokenFactoryReset(token_factory_reset::Response),
}
