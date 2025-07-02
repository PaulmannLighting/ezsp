use std::future::Future;

use crate::Error;
use crate::parameters::configuration::version;

pub use binding::Binding;
pub use bootloader::Bootloader;
pub use cbke::Cbke;
pub use configuration::{Configuration, GetValueExt};
pub use green_power::{GreenPower, ProxyTable, SinkTable};
pub use messaging::Messaging;
pub use mfglib::Mfglib;
pub use networking::Networking;
pub use security::Security;
pub use token_interface::TokenInterface;
pub use trust_center::TrustCenter;
pub use utilities::Utilities;
pub use wwah::Wwah;
pub use zll::Zll;

mod binding;
mod bootloader;
mod cbke;
mod configuration;
mod green_power;
mod messaging;
mod mfglib;
mod networking;
mod security;
mod token_interface;
mod trust_center;
mod utilities;
mod wwah;
mod zll;

/// A trait to represent implementors of the full `EZSP` protocol.
///
/// This trait is a convenience trait that combines all the other EZSP traits.
pub trait Ezsp:
    Binding
    + Bootloader
    + Cbke
    + Configuration
    + GreenPower
    + Messaging
    + Mfglib
    + Networking
    + Security
    + TokenInterface
    + TrustCenter
    + Utilities
    + Wwah
    + Zll
{
    /// Initialize the `EZSP` connection.
    ///
    /// Negotiates the protocol version.
    ///
    /// TODO: Maybe restore network from exported settings here, too?
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] on I/O errors or if the desired protocol version is not supported.
    fn init(&mut self) -> impl Future<Output = Result<version::Response, Error>> + Send;
}
