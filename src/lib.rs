//! The `Ember ZNet Serial Protocol` (`EZSP`)
//!
//! This library implements the `Ember ZNet Serial Protocol`, `EZSP` for short.
//! You can find the protocol's definition on [siliconlabs.com](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![deny(unsafe_code)]

mod constants;
pub mod ember;
mod error;
pub mod ezsp;
mod frame;
mod handler;
mod result;
mod transport;
mod types;
#[cfg(feature = "ashv2")]
pub mod uart;
mod zigbee;

pub use constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};
pub use error::{Error, ValueError};
pub use frame::{
    parameters, Callback, CallbackType, Extended, FormatVersion, Frame, Header, HighByte, Legacy,
    LowByte, Parameters, Parsable, Response, SleepMode,
};
pub use handler::Handler;
pub use result::Result;
pub use transport::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GreenPower, Messaging, Mfglib, Networking,
    ProxyTable, Security, SinkTable, TokenInterface, Transport, TrustCenter, Utilities, Wwah, Zll,
    MIN_NON_LEGACY_VERSION,
};
pub use types::SourceRouteDiscoveryMode;
