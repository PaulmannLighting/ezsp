use crate::error::Resolve;
use crate::frame::parameters::token_interface::get_token_count;
use crate::Transport;
use std::future::Future;

pub trait TokenInterface {
    /// Gets the total number of tokens.
    fn get_token_count(&self) -> impl Future<Output = Result<u8, crate::Error>> + Send;
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
}
