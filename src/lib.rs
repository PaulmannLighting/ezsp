//! The `Ember ZNet Serial Protocol` (`EZSP`)
//!
//! This library implements the `Ember ZNet Serial Protocol`, `EZSP` for short.
//! You can find the protocol's definition on [siliconlabs.com](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![deny(unsafe_code)]
extern crate core;

pub use self::commands::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GetValueExt, GreenPower, Messaging, Mfglib,
    Networking, ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
pub use self::constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE, MIN_NON_LEGACY_VERSION};
pub use self::defragmentation::{Defragment, DefragmentationError, DefragmentedMessage};
pub use self::error::{Error, ValueError};
pub use self::extensions::{ConfigurationExt, Displayable, PolicyExt};
pub use self::frame::{
    Callback, CallbackType, Command, Disambiguation, Extended, FormatVersion, Frame, Header,
    HighByte, Legacy, LowByte, Parameters, Parsable, Response, SleepMode, parameters,
};
pub use self::result::Result;
pub use self::transport::Transport;
pub use self::types::SourceRouteDiscoveryMode;

mod commands;
mod constants;
mod defragmentation;
pub mod ember;
mod error;
mod extensions;
pub mod ezsp;
mod frame;
mod result;
mod transport;
mod types;
#[cfg(feature = "ashv2")]
pub mod uart;
#[cfg(feature = "zigbee")]
pub mod zigbee;
