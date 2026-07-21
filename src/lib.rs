//! Host-side support for the `EmberZNet` Serial Protocol (`EZSP`).
//!
//! EZSP is the protocol used by a host application processor to communicate
//! with the `EmberZNet` PRO stack running on a Network Co-Processor (`NCP`).
//! Commands are sent by the host and answered by the `NCP`; UART-based NCPs may
//! additionally send asynchronous callbacks as they occur.
//!
//! This crate provides typed EZSP frame headers, parameter structures, and
//! command traits on top of an actor-based transport. A [`Transceiver`] joins
//! independent [`Transmit`] and [`Receive`] implementations and spawns tasks
//! that serialize outgoing work, correlate responses by EZSP sequence number,
//! and route asynchronous callbacks separately. After protocol negotiation, a
//! cloneable [`Connected`] handle implements [`Communicate`] and therefore all
//! command-group traits.
//!
//! [`Builder`] orchestrates actor startup, protocol negotiation, stack
//! configuration, network restoration or formation, endpoint registration,
//! and callback translation. The resulting [`Ncp`] adds higher-level scan and
//! APS messaging workflows. With the `ashv2` feature enabled,
//! `Builder::ashv2` supplies concrete transport halves for EZSP over `ASHv2`,
//! and the `ashv2` crate is re-exported for its serial-port API.
//!
//! Protocol details are documented by Silicon Labs in the
//! [Simplicity SDK EZSP Reference Guide](https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![deny(unsafe_code)]

#[cfg(feature = "ashv2")]
pub use ashv2;

pub use self::commands::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GetValueExt, GreenPower, Messaging, Mfglib,
    Networking, ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
pub use self::communicate::Communicate;
pub use self::connection::Connection;
pub use self::constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE, MIN_NON_LEGACY_VERSION};
pub use self::defragmentation::{Defragmented, DefragmentedMessage, Defragmenter};
pub use self::error::{Error, ValueError};
pub use self::extensions::{ConfigurationExt, Displayable, PolicyExt};
pub use self::frame::{
    Callback, CallbackType, Command, Commands, Extended, FormatVersion, Frame, Header, HighByte,
    Legacy, LowByte, Parameters, Parsable, Response, SleepMode, parameters,
};
pub use self::ncp::{
    Builder, Endpoint, EventHandler, InitializationParameters, MulticastOptions, Ncp,
    NetworkCredentials, Scans, StackResponse, Startup,
};
pub use self::transceiver::{
    Connected, Disconnected, Receive, Receiver, Transceiver, TranslatableEvent, Transmit,
    Transmitter,
};
pub use self::types::SourceRouteDiscoveryMode;

#[cfg(feature = "apis-saltans")]
pub mod apis_saltans;
mod commands;
mod communicate;
mod connection;
mod constants;
mod defragmentation;
pub mod ember;
mod error;
mod extensions;
pub mod ezsp;
mod frame;
mod ncp;
mod transceiver;
mod types;
#[cfg(feature = "ashv2")]
mod uart;

/// A specialized [`std::result::Result`] type for this crate.
pub type Result<T> = core::result::Result<T, Error>;
