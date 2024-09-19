use crate::ember::token;
use crate::error::Resolve;
use crate::frame::parameters::token_interface::{get_token_count, get_token_data, get_token_info};
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
}
