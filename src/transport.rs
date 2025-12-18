//! Transport layer to communicate with an NCP that supports the `EZSP` protocol.

use core::future::Future;

use le_stream::ToLeStream;

use crate::frame::Parameter;
use crate::{Error, Parameters};

/// The minimum protocol version that supports non-legacy commands.
pub const MIN_NON_LEGACY_VERSION: u8 = 8;

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Check if the `EZSP` connection needs to be reset and reset it if necessary.
    fn check_reset(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

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
            self.check_reset().await?;
            self.send::<C>(command).await?;
            self.receive().await
        }
    }
}
