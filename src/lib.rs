//! The `Ember ZNet Serial Protocol` (`EZSP`)
//!
//! This library implements the `Ember ZNet Serial Protocol`, `EZSP` for short.
//! You can find the protocol's definition on [siliconlabs.com](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf).
//!
//! This library is free software and is not affiliated with Silicon Labs.
#[cfg(feature = "ashv2")]
pub mod ashv2;
mod bootoader;
mod constants;
pub mod ember;
mod error;
pub mod ezsp;
mod frame;
mod result;
mod transport;
mod types;

pub use constants::{MAX_FRAME_SIZE, MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};
pub use error::{Error, ValueError};
#[cfg(feature = "responses")]
pub use frame::response::Response;
pub use frame::{
    parameters, CallbackType, Extended, FormatVersion, Frame, Handler, Header, HighByte, Legacy,
    LowByte, Parsable, SleepMode,
};
pub use result::Result;
pub use transport::{
    Binding, Bootloader, Cbke, Configuration, Ezsp, GreenPower, Messaging, Mfglib, Networking,
    ProxyTable, Security, SinkTable, TokenInterface, Transport, TrustCenter, Utilities, Wwah, Zll,
};
pub use types::SourceRouteDiscoveryMode;
