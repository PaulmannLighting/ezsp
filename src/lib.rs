//! The Ember ZNet Serial Protocol (`EZSP`)
//!
//! This library implements the Ember ZNet Serial Protocol, `EZSP` for short.
//! You can find the protocol's definition on [siliconlabs.com](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf).
//!
//! This library is free software and is not affiliated with Silicon Labs.
pub mod ember;
pub mod error;
pub mod ezsp;
pub mod frame;
mod resolve;
mod result;
pub mod transport;
pub mod types;

pub use error::{value::Error as ValueError, Error};
pub use frame::{CallbackType, Control, FrameFormatVersion, Header, HighByte, LowByte, SleepMode};
pub(crate) use resolve::Resolve;
pub use result::Result;
#[cfg(feature = "ashv2")]
pub use transport::Ashv2;
pub use transport::{
    Binding, Bootloader, CertificateBasedKeyExchange, Configuration, Ezsp, GreenPower, Messaging,
    Mfglib, Networking, ProxyTable, Security, SinkTable, TokenInterface, Transport, TrustCenter,
    Utilities, Wwah, Zll,
};
