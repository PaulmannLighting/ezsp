use std::future::Future;

use crate::error::Resolve;
use crate::frame::parameters::mfglib::{end, get_channel};
use crate::{Error, Transport};

pub trait Mfglib {
    /// Deactivate use of `Mfglib` test routines; restores the hardware to the state it was in prior
    /// to [`start()`](Self::start) and stops receiving packets started by [`start()`](Self::start)
    /// at the same time.
    fn end(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Returns the current radio channel, as previously set via [`set_channel()`](Self::set_channel).
    fn get_channel(&self) -> impl Future<Output = Result<u8, Error>> + Send;
}

impl<T> Mfglib for T
where
    T: Transport,
{
    async fn end(&self) -> Result<(), Error> {
        self.communicate::<_, end::Response>(end::Command)
            .await?
            .resolve()
    }

    async fn get_channel(&self) -> Result<u8, Error> {
        self.communicate::<_, get_channel::Response>(get_channel::Command)
            .await
            .map(|response| response.channel())
    }
}
