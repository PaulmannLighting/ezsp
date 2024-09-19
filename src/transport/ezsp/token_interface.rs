use crate::ember::token;
use crate::error::Resolve;
use crate::frame::parameters::token_interface::{
    get_token_count, get_token_data, get_token_info, gp_security_test_vectors, reset_node,
};
use crate::Transport;
use std::future::Future;

pub trait TokenInterface {
    /// Gets the total number of tokens.
    fn get_token_count(&self) -> impl Future<Output = Result<u8, crate::Error>> + Send;

    /// Gets the token data for a single token with provided key.
    fn get_token_data(
        &self,
        token: u32,
        index: u32,
    ) -> impl Future<Output = Result<token::Data, crate::Error>> + Send;

    /// Gets the token information for a single token at provided index
    fn get_token_info(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<token::Info, crate::Error>> + Send;

    /// Run GP security test vectors.
    fn gp_security_test_vectors(&self) -> impl Future<Output = Result<(), crate::Error>> + Send;

    /// Reset the node by calling `halReboot`.
    fn reset_node(&self) -> impl Future<Output = Result<(), crate::Error>> + Send;
}

impl<T> TokenInterface for T
where
    T: Transport,
{
    async fn get_token_count(&self) -> Result<u8, crate::Error> {
        self.communicate::<_, get_token_count::Response>(get_token_count::Command)
            .await?
            .resolve()
    }

    async fn get_token_data(&self, token: u32, index: u32) -> Result<token::Data, crate::Error> {
        self.communicate::<_, get_token_data::Response>(get_token_data::Command::new(token, index))
            .await?
            .resolve()
    }

    async fn get_token_info(&self, index: u8) -> Result<token::Info, crate::Error> {
        self.communicate::<_, get_token_info::Response>(get_token_info::Command::new(index))
            .await?
            .resolve()
    }

    async fn gp_security_test_vectors(&self) -> Result<(), crate::Error> {
        self.communicate::<_, gp_security_test_vectors::Response>(gp_security_test_vectors::Command)
            .await?
            .resolve()
    }

    async fn reset_node(&self) -> Result<(), crate::Error> {
        self.communicate::<_, reset_node::Response>(reset_node::Command)
            .await?
            .resolve()
    }
}
