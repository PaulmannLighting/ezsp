use std::future::Future;

use crate::{Error, Transport};

pub trait Security: Transport {
    /// This function clears the key table of the current network.
    fn clear_key_table(&self) -> impl Future<Output = Result<(), Error>>;
}
