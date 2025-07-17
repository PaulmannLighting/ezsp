use core::future::Future;

use crate::Transport;
use crate::ember::token::{Data, Info};
use crate::frame::parameters::token_interface::{
    get_token_count, get_token_data, get_token_info, gp_security_test_vectors, reset_node,
    set_token_data, token_factory_reset,
};

/// The `TokenInterface` trait provides an interface for the token interface.
pub trait TokenInterface {
    /// Gets the total number of tokens.
    fn get_token_count(&mut self) -> impl Future<Output = Result<u8, crate::Error>> + Send;

    /// Gets the token data for a single token with provided key.
    fn get_token_data(
        &mut self,
        token: u32,
        index: u32,
    ) -> impl Future<Output = Result<Data, crate::Error>> + Send;

    /// Gets the token information for a single token at provided index
    fn get_token_info(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<Info, crate::Error>> + Send;

    /// Run GP security test vectors.
    fn gp_security_test_vectors(&mut self)
    -> impl Future<Output = Result<(), crate::Error>> + Send;

    /// Reset the node by calling `halReboot`.
    fn reset_node(&mut self) -> impl Future<Output = Result<(), crate::Error>> + Send;

    /// Sets the token data for a single token with provided key.
    fn set_token_data(
        &mut self,
        token: u32,
        index: u32,
        token_data: Data,
    ) -> impl Future<Output = Result<(), crate::Error>> + Send;

    /// Factory reset all configured Zigbee tokens.
    fn token_factory_reset(
        &mut self,
        exclude_outgoing_fc: bool,
        exclude_boot_counter: bool,
    ) -> impl Future<Output = Result<(), crate::Error>> + Send;
}

impl<T> TokenInterface for T
where
    T: Transport,
{
    async fn get_token_count(&mut self) -> Result<u8, crate::Error> {
        self.communicate::<_, get_token_count::Response>(get_token_count::Command)
            .await
            .map(Into::into)
    }

    async fn get_token_data(&mut self, token: u32, index: u32) -> Result<Data, crate::Error> {
        self.communicate::<_, get_token_data::Response>(get_token_data::Command::new(token, index))
            .await?
            .try_into()
    }

    async fn get_token_info(&mut self, index: u8) -> Result<Info, crate::Error> {
        self.communicate::<_, get_token_info::Response>(get_token_info::Command::new(index))
            .await?
            .try_into()
    }

    async fn gp_security_test_vectors(&mut self) -> Result<(), crate::Error> {
        self.communicate::<_, gp_security_test_vectors::Response>(gp_security_test_vectors::Command)
            .await?
            .try_into()
    }

    async fn reset_node(&mut self) -> Result<(), crate::Error> {
        self.communicate::<_, reset_node::Response>(reset_node::Command)
            .await
            .map(drop)
    }

    async fn set_token_data(
        &mut self,
        token: u32,
        index: u32,
        token_data: Data,
    ) -> Result<(), crate::Error> {
        self.communicate::<_, set_token_data::Response>(set_token_data::Command::new(
            token, index, token_data,
        ))
        .await?
        .try_into()
    }

    async fn token_factory_reset(
        &mut self,
        exclude_outgoing_fc: bool,
        exclude_boot_counter: bool,
    ) -> Result<(), crate::Error> {
        self.communicate::<_, token_factory_reset::Response>(token_factory_reset::Command::new(
            exclude_outgoing_fc,
            exclude_boot_counter,
        ))
        .await
        .map(drop)
    }
}
