//! High-level EZSP command traits.
//!
//! Silicon Labs groups EZSP frames by protocol area, such as configuration,
//! utilities, networking, messaging, security, and bootloader support. This
//! module exposes one async trait per group plus [`Ezsp`], a convenience trait
//! for communicators that implement the complete command surface represented
//! by this crate.

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

/// Convenience trait for implementors of the full EZSP command surface.
///
/// Implementing [`Communicate`](crate::Communicate) is enough to get blanket
/// implementations of the individual command-group traits. Every
/// [`Transport`](crate::Transport) implements `Communicate` automatically. This
/// trait simply collects the command groups under one bound.
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
}

impl<T> Ezsp for T where
    T: Binding
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
}
