use std::future::Future;

use siliconlabs;
use siliconlabs::zigbee::security::ManContext;

use crate::error::Resolve;
use crate::frame::parameters::security::{
    check_key_context, clear_key_table, clear_transient_link_keys,
};
use crate::{Error, Transport};

pub trait Security {
    /// Check whether a key context can be used to load a valid key.
    fn check_key_context(
        &self,
        context: ManContext,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function clears the key table of the current network.
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clear all the transient link keys from RAM.
    fn clear_transient_link_keys(&self) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Security for T
where
    T: Transport,
{
    async fn check_key_context(&self, context: ManContext) -> Result<(), Error> {
        self.communicate::<_, check_key_context::Response>(check_key_context::Command::new(context))
            .await?
            .resolve()
    }

    async fn clear_key_table(&self) -> Result<(), Error> {
        self.communicate::<_, clear_key_table::Response>(clear_key_table::Command)
            .await?
            .resolve()
    }

    async fn clear_transient_link_keys(&self) -> Result<(), Error> {
        self.communicate::<_, clear_transient_link_keys::Response>(
            clear_transient_link_keys::Command,
        )
        .await
        .map(drop)
    }
}
