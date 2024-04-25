use std::future::Future;

use crate::error::Resolve;
use crate::frame::parameters::security::{clear_key_table, clear_transient_link_keys};
use crate::{Error, Transport};

pub trait Security {
    /// This function clears the key table of the current network.
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>>;

    /// Clear all the transient link keys from RAM.
    fn clear_transient_link_keys(&self) -> impl Future<Output = Result<(), Error>>;
}

impl<T> Security for T
where
    T: Transport,
{
    async fn clear_key_table(&self) -> Result<(), Error> {
        self.communicate::<_, clear_key_table::Response>(clear_key_table::Command)
            .await?
            .status()
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
