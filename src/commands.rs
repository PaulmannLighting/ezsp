use core::future::Future;

pub use self::binding::Binding;
pub use self::bootloader::Bootloader;
pub use self::cbke::Cbke;
pub use self::configuration::{Configuration, GetValueExt};
pub use self::green_power::{GreenPower, ProxyTable, SinkTable};
pub use self::messaging::Messaging;
pub use self::mfglib::Mfglib;
pub use self::networking::Networking;
pub use self::security::Security;
pub use self::token_interface::TokenInterface;
pub use self::trust_center::TrustCenter;
pub use self::utilities::Utilities;
pub use self::wwah::Wwah;
pub use self::zll::Zll;
use crate::Error;
use crate::parameters::configuration::version;

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
    /// # Errors
    ///
    /// Returns an [`Error`] on I/O errors or if the desired protocol version is not supported.
    fn init(&mut self) -> impl Future<Output = Result<version::Response, Error>> + Send;

    /// Returns the negotiated protocol version, if any.
    fn negotiated_version(&self) -> Option<u8>;
}
