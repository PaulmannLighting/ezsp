use core::future::Future;

use le_stream::ToLeStream;

use crate::frame::Parameter;
use crate::{Error, Transport};

/// Sends typed EZSP command parameters without waiting for their response.
///
/// This is the transmitting counterpart to [`Receive`](super::Receive). Its
/// method has the same command bounds and result as [`Transport::send`].
pub trait Transmit: Send {
    /// Transmits one EZSP command frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when the command cannot be transmitted.
    fn transmit<T>(&mut self, command: T) -> impl Future<Output = Result<u16, Error>> + Send
    where
        T: Parameter + ToLeStream;
}

impl<T> Transmit for T
where
    T: Transport,
{
    async fn transmit<U>(&mut self, command: U) -> Result<u16, Error>
    where
        U: Parameter + ToLeStream,
    {
        Transport::send(self, command).await
    }
}
