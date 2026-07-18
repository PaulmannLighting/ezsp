//! Host-side support for the `EmberZNet` Serial Protocol (`EZSP`).
//!
//! EZSP is the protocol used by a host application processor to communicate
//! with the `EmberZNet` PRO stack running on a Network Co-Processor (`NCP`).
//! Commands are sent by the host and answered by the `NCP`; UART-based NCPs may
//! additionally send asynchronous callbacks as they occur.
//!
//! This crate provides typed EZSP frame headers, parameter structures,
//! command/response traits, and transport-independent command traits. With the
//! `ashv2` feature enabled, it also provides a UART transport for EZSP over
//! `ASHv2`; the [`uart`] module re-exports the `ASHv2` types used by that public
//! transport API.
//!
//! Protocol details are documented by Silicon Labs in the
//! [Simplicity SDK EZSP Reference Guide](https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![deny(unsafe_code)]

pub use self::commands::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GetValueExt, GreenPower, Messaging, Mfglib,
    Networking, ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
pub use self::connection::Connection;
pub use self::constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE, MIN_NON_LEGACY_VERSION};
pub use self::defragmentation::{Defragmented, DefragmentedMessage, Defragmenter};
pub use self::error::{Error, ValueError};
pub use self::extensions::{ConfigurationExt, Displayable, PolicyExt};
pub use self::frame::{
    Callback, CallbackType, Command, Extended, FormatVersion, Frame, Header, HighByte, Legacy,
    LowByte, Parameters, Parsable, Response, SleepMode, parameters,
};
pub use self::ncp::{Builder, Message, MulticastOptions, Ncp, Scans};
pub use self::result::Result;
pub use self::transport::Transport;
pub use self::types::SourceRouteDiscoveryMode;

#[cfg(feature = "apis-saltans")]
pub mod apis_saltans;
mod commands;
mod connection;
mod constants;
mod defragmentation;
pub mod ember;
mod error;
mod extensions;
pub mod ezsp;
mod frame;
mod ncp;
mod result;
mod shared_transport;
mod transport;
mod types;
#[cfg(feature = "ashv2")]
pub mod uart;
