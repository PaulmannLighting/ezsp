#[cfg(feature = "ashv2")]
pub(crate) mod ashv2;
mod ezsp;

use crate::frame::{Extended, Header, Parameter};
use crate::Error;
#[cfg(feature = "ashv2")]
pub use ashv2::Ashv2;
pub use ezsp::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Ezsp, GreenPower, Messaging,
    Mfglib, Networking, ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities,
    Wwah, Zll,
};
use le_stream::{FromLeStream, ToLeStream};
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Return the next header.
    fn next_header<H, T>(&mut self, id: T) -> H
    where
        H: Header<T>,
        T: Copy + Clone + Debug + Eq + Hash + Into<u16> + PartialEq + Send;

    /// Send a command to the NCP.
    fn send<H, P>(&mut self, command: P) -> impl Future<Output = Result<(), Error>> + Send
    where
        H: Header<P::Id>,
        P: Parameter + ToLeStream;

    /// Receive a response from the NCP.
    fn receive<H, P>(&mut self) -> impl Future<Output = Result<P, Error>> + Send
    where
        H: Header<P::Id>,
        P: Parameter + FromLeStream;

    /// Communicate with the NCP.
    ///
    /// This assumes that `C::ID` and `R::ID` are the same.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Parameter<Id = u16> + ToLeStream,
        R: Clone + Debug + Parameter<Id = u16> + FromLeStream,
    {
        async {
            self.send::<Extended, C>(command).await?;
            self.receive::<Extended, R>().await
        }
    }
}
