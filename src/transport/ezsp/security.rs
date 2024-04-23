use std::future::Future;

use crate::Error;

pub trait Security {
    /// This function clears the key table of the current network.
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>>;

    /// Clear all the transient link keys from RAM.
    fn clear_transient_link_keys(&self) -> impl Future<Output = Result<(), Error>>;
}
