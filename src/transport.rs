#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;

use crate::frame::Parameter;
use crate::{Error, Header};
#[cfg(feature = "ashv2")]
pub use ashv2::Ashv2;
pub use ezsp::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Ezsp, Messaging, Networking,
    TrustCenter,
};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;
use std::future::Future;

pub trait Transport {
    fn next_header<R>(&self) -> Header<R::Id>
    where
        R: Parameter;

    fn communicate<C, R>(&self, command: C) -> impl Future<Output = Result<R, Error>>
    where
        Self: 'static,
        C: Parameter + ToLeBytes,
        for<'response> R: Clone + Debug + Send + Sync + Parameter + FromLeBytes + 'response;
}
