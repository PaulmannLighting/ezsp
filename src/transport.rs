//! Transport layer to communicate with an NCP that supports the `EZSP` protocol.

mod ezsp;

use crate::frame::{Header, Identified};
use crate::{Error, Parameters};
pub use ezsp::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GreenPower, Messaging, Mfglib, Networking,
    ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
use le_stream::ToLeStream;
use std::future::Future;
use std::num::TryFromIntError;

pub const MIN_NON_LEGACY_VERSION: u8 = 8;

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Return the next header.
    ///
    /// This method is used to determine the next header to be used in the communication.
    ///
    /// The `id` parameter is the identifier of the command that will be sent.
    ///
    /// # Errors
    ///
    /// This method may return an error if `EZSP` is in legacy mode
    /// and the `id` cannot be converted into a `u8`.
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError>;

    /// Check if the `EZSP` connection needs to be reset and reset it if necessary.
    fn check_reset(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a command to the NCP.
    fn send<T>(&mut self, command: T) -> impl Future<Output = Result<(), Error>> + Send
    where
        T: Identified + ToLeStream;

    /// Receive a response from the NCP.
    fn receive<T>(&mut self) -> impl Future<Output = Result<T, Error>> + Send
    where
        T: TryFrom<Parameters>,
        Error: From<<T as TryFrom<Parameters>>::Error>;

    /// Communicate with the NCP.
    ///
    /// This assumes that `C::ID` and `R::ID` are the same.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Identified + ToLeStream,
        R: TryFrom<Parameters>,
        Error: From<<R as TryFrom<Parameters>>::Error>,
    {
        async {
            self.check_reset().await?;
            self.send::<C>(command).await?;
            self.receive().await
        }
    }
}
