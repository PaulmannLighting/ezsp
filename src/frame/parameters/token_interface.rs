//! Token Interface Frames

use le_stream::FromLeStream;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::Parameter;

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

impl Parsable for Response {
    fn parse_from_le_stream<T>(id: u16, stream: T) -> Result<Self, Decode>
    where
        T: Iterator<Item = u8>,
    {
        match id {
            <get_token_count::Response as Parameter>::ID => Ok(Self::GetTokenCount(
                get_token_count::Response::from_le_stream_exact(stream)?,
            )),
            <get_token_data::Response as Parameter>::ID => Ok(Self::GetTokenData(
                get_token_data::Response::from_le_stream_exact(stream)?,
            )),
            <get_token_info::Response as Parameter>::ID => Ok(Self::GetTokenInfo(
                get_token_info::Response::from_le_stream_exact(stream)?,
            )),
            <gp_security_test_vectors::Response as Parameter>::ID => {
                Ok(Self::GpSecurityTestVectors(
                    gp_security_test_vectors::Response::from_le_stream_exact(stream)?,
                ))
            }
            <reset_node::Response as Parameter>::ID => Ok(Self::ResetNode(
                reset_node::Response::from_le_stream_exact(stream)?,
            )),
            <set_token_data::Response as Parameter>::ID => Ok(Self::SetTokenData(
                set_token_data::Response::from_le_stream_exact(stream)?,
            )),
            <token_factory_reset::Response as Parameter>::ID => Ok(Self::TokenFactoryReset(
                token_factory_reset::Response::from_le_stream_exact(stream)?,
            )),
            _ => Err(Decode::InvalidFrameId(id)),
        }
    }
}
