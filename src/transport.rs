#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;

use crate::frame::{Command, Extended, Parameter, Response, ValidControl};
use crate::{Error, Header};
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

/// A transport layer to communicate with an NCP that supports the `EZSP` protocol.
///
/// This trait is intended to facilitate the implementation of the `EZSP` protocol,
/// since all `EZSP` traits have a blanket implementation for it.
///
/// Unless you know what you are doing, you should not use the methods of this trait directly.
pub trait Transport: Send {
    /// Return the next header.
    fn next_header<T>(&mut self, id: T::Size) -> Header<T>
    where
        T: ValidControl;

    /// Send a command to the NCP.
    fn send<C, P>(&mut self, command: P) -> impl Future<Output = Result<(), Error>> + Send
    where
        C: ValidControl,
        P: Parameter + ToLeStream,
        <P as Parameter>::Id: Into<C::Size>;

    /// Receive a raw response from the NCP.
    fn receive_raw<C, R>(&mut self) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: ValidControl,
        R: Clone + Debug + FromLeStream;

    /// Receive a response from the NCP.
    fn receive<C, P>(&mut self) -> impl Future<Output = Result<P, Error>> + Send
    where
        C: ValidControl,
        P: Clone + Debug + Parameter + FromLeStream,
        <P as Parameter>::Id: Into<C::Size>;

    /// Communicate with the NCP.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Parameter + ToLeStream,
        R: Clone + Debug + Parameter + FromLeStream,
        <C as Parameter>::Id: Into<<Extended<Command> as ValidControl>::Size>,
        <R as Parameter>::Id: Into<<Extended<Response> as ValidControl>::Size>,
    {
        async {
            self.send::<Extended<Command>, C>(command).await?;
            self.receive::<Extended<Response>, R>().await
        }
    }
}
