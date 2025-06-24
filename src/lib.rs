//! The `Ember ZNet Serial Protocol` (`EZSP`)
//!
//! This library implements the `Ember ZNet Serial Protocol`, `EZSP` for short.
//! You can find the protocol's definition on [siliconlabs.com](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![deny(unsafe_code)]

mod commands;
mod constants;
pub mod ember;
mod error;
pub mod ezsp;
mod frame;
mod result;
mod transport;
mod types;
#[cfg(feature = "ashv2")]
pub mod uart;
mod zigbee;

pub use commands::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GreenPower, Messaging, Mfglib, Networking,
    ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
pub use constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};
pub use error::{Error, ValueError};
pub use frame::{
    Callback, CallbackType, Extended, FormatVersion, Frame, Header, HighByte, Legacy, LowByte,
    Parameters, Parsable, Response, SleepMode, parameters,
};
pub use result::Result;
pub use transport::{MIN_NON_LEGACY_VERSION, Transport};
pub use types::SourceRouteDiscoveryMode;
