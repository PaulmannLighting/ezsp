//! Host-side support for the `EmberZNet` Serial Protocol (`EZSP`).
//!
//! EZSP is the protocol used by a host application processor to communicate
//! with the `EmberZNet` PRO stack running on a Network Co-Processor (`NCP`).
//! Commands are sent by the host and answered by the `NCP`; UART-based NCPs may
//! additionally send asynchronous callbacks as they occur.
//!
//! This crate provides typed EZSP frame headers, parameter structures, and
//! command traits on top of an actor-based transport. Independent [`Transmit`]
//! and [`Receive`] implementations are driven by futures that the caller spawns
//! as tasks. Those actors serialize outgoing work, correlate responses by EZSP
//! sequence number, and route asynchronous callbacks separately. After protocol
//! negotiation, a cloneable [`Connection`] handle implements [`Communicate`]
//! and therefore all command-group traits.
//!
//! [`Builder`] orchestrates actor startup, protocol negotiation, stack
//! configuration, network restoration or formation, endpoint registration,
//! and callback translation. The resulting [`Ncp`] adds higher-level scan and
//! APS messaging workflows.
//!
//! This crate does not supply an `ASHv2` implementation. An external `ASHv2`
//! adapter implements [`Transmit`] and [`Receive`], passes both halves to
//! [`Client::run`], and spawns the returned [`Futures`] before constructing
//! [`Builder`] from the newly wired [`Client`].
//!
//! Protocol details are documented by Silicon Labs in the
//! [Simplicity SDK EZSP Reference Guide](https://docs.silabs.com/zigbee/latest/sisdk-ezsp-reference-guide/).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(unsafe_code)]

pub use self::api::{Client, Connection, Futures, Receive, TranslatableEvent, Transmit};
pub use self::commands::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GetValueExt, GreenPower, Messaging, Mfglib,
    Networking, ProxyTable, Security, SinkTable, TokenInterface, TrustCenter, Utilities, Wwah, Zll,
};
pub use self::communicate::Communicate;
pub use self::constants::{MAX_HEADER_SIZE, MIN_NON_LEGACY_VERSION};
pub use self::defragmentation::{Defragmented, DefragmentedMessage, Defragmenter};
pub use self::error::{Decode, Error, Status, ValueError};
pub use self::extensions::{ConfigurationExt, Displayable, PolicyExt};
pub use self::frame::{
    Callback, CallbackType, Command, Commands, Extended, FormatVersion, Frame, Header, HighByte,
    Legacy, LowByte, Parameters, Parsable, Response, SleepMode, parameters,
};
pub use self::ncp::{
    BuildResult, Builder, Endpoint, EventHandler, InitializationParameters, MulticastOptions, Ncp,
    NetworkCredentials, Scans, StackResponse, Startup,
};
pub use self::types::SourceRouteDiscoveryMode;

mod api;
#[cfg(feature = "apis-saltans")]
#[cfg_attr(docsrs, doc(cfg(feature = "apis-saltans")))]
pub mod apis_saltans;
mod commands;
mod communicate;
mod constants;
mod defragmentation;
pub mod ember;
mod error;
mod extensions;
pub mod ezsp;
mod frame;
mod ncp;
mod types;

/// A specialized [`std::result::Result`] type for this crate.
pub type Result<T> = core::result::Result<T, Error>;
