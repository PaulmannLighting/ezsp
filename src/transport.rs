#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;
mod parse_response;

use crate::frame::{Extended, Parameter, ValidControl};
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
pub trait Transport: Send {
    /// Return the next header.
    fn next_header<T>(&mut self, id: T::Size) -> Header<T>
    where
        T: ValidControl;

    fn send<C, P>(&mut self, command: P) -> impl Future<Output = Result<(), Error>> + Send
    where
        C: ValidControl,
        P: Parameter + ToLeStream,
        <P as Parameter>::Id: Into<C::Size>;

    fn receive<C, P>(&mut self) -> impl Future<Output = Result<P, Error>> + Send
    where
        C: ValidControl,
        P: Clone + Debug + Send + Parameter + FromLeStream,
        <P as Parameter>::Id: Into<C::Size>;

    /// Communicate with the NCP.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Parameter + ToLeStream,
        R: Clone + Debug + Send + Parameter + FromLeStream,
        <C as Parameter>::Id: Into<<Extended as ValidControl>::Size>,
        <R as Parameter>::Id: Into<<Extended as ValidControl>::Size>,
    {
        async {
            self.send::<Extended, C>(command).await?;
            self.receive::<Extended, R>().await
        }
    }
}
