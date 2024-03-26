#[cfg(feature = "ashv2")]
mod ashv2;
mod ezsp;

use crate::frame::Parameter;
use crate::{Error, Header};
#[cfg(feature = "ashv2")]
pub use ashv2::Ashv2;
pub use ezsp::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Ezsp, Messaging, TrustCenter,
};
use le_stream::{FromLeBytes, ToLeBytes};
use std::fmt::Debug;
use std::future::Future;

pub trait Transport {
    fn next_header<R>(&mut self) -> Header<R::Id>
    where
        R: Parameter;

    fn communicate<R>(&mut self, command: impl Parameter) -> impl Future<Output = Result<R, Error>>
    where
        for<'r> R: Clone + Debug + Parameter + Send + Sync + 'r;
}
