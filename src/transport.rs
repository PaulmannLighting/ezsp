#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;
mod parse_response;

use crate::frame::Parameter;
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
    fn next_header<R>(&mut self) -> Header<R::Id>
    where
        R: Parameter;

    /// Communicate with the NCP.
    fn communicate<C, R>(&mut self, command: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: Parameter + ToLeStream,
        R: Clone + Debug + Send + Parameter + FromLeStream;
}
