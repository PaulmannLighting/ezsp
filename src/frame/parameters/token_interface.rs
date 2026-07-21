//! Token Interface Frames

pub use self::get_token_count::Response as GetTokenCount;
pub use self::get_token_data::Response as GetTokenData;
pub use self::get_token_info::Response as GetTokenInfo;
pub use self::gp_security_test_vectors::Response as GpSecurityTestVectors;
pub use self::reset_node::Response as ResetNode;
pub use self::set_token_data::Response as SetTokenData;
pub use self::token_factory_reset::Response as TokenFactoryReset;

pub mod get_token_count;
pub mod get_token_data;
pub mod get_token_info;
pub mod gp_security_test_vectors;
pub mod reset_node;
pub mod set_token_data;
pub mod token_factory_reset;

crate::frame::parameters::command_enum!(
    Command,
    GetTokenCount(get_token_count::Command),
    GetTokenData(get_token_data::Command),
    GetTokenInfo(get_token_info::Command),
    GpSecurityTestVectors(gp_security_test_vectors::Command),
    ResetNode(reset_node::Command),
    SetTokenData(set_token_data::Command),
    TokenFactoryReset(token_factory_reset::Command),
);

crate::frame::parameters::parameter_enum!(
    Response,
    GetTokenCount,
    GetTokenData,
    GetTokenInfo,
    GpSecurityTestVectors,
    ResetNode,
    SetTokenData,
    TokenFactoryReset
);
