//! Transport layer to communicate with an NCP that supports the `EZSP` protocol.

use core::future::Future;

use le_stream::ToLeStream;

use crate::frame::Parameter;
use crate::{Error, Parameters};

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Ensure that an `EZSP` connection is established and reset it if necessary.
    fn ensure_connection(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a command to the NCP.
    fn send<T>(&mut self, command: T) -> impl Future<Output = Result<u16, Error>> + Send
    where
        T: Parameter + ToLeStream;

    /// Receive a response from the NCP.
    fn receive<T>(&mut self) -> impl Future<Output = Result<T, Error>> + Send
    where
        T: TryFrom<Parameters> + Send,
        <T as TryFrom<Parameters>>::Error: Into<Parameters> + Send;

    /// Communicate with the NCP.
    ///
    /// This assumes that `C::ID` and `R::ID` are the same.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Parameter + ToLeStream,
        R: TryFrom<Parameters> + Send,
        <R as TryFrom<Parameters>>::Error: Into<Parameters> + Send,
    {
        async {
            self.ensure_connection().await?;
            self.send::<C>(command).await?;
            self.receive().await
        }
    }
}
