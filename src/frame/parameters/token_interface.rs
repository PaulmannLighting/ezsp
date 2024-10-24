//! Token Interface Frames

pub mod get_token_count;
pub mod get_token_data;
pub mod get_token_info;
pub mod gp_security_test_vectors;
pub mod reset_node;
pub mod set_token_data;
pub mod token_factory_reset;

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
