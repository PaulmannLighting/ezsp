//! Integration with the `apis-saltans` Zigbee hardware traits.
//!
//! Enabling the `apis-saltans` feature adapts [`crate::Ncp`] and
//! [`crate::Builder`] to the `apis_saltans_hw` API:
//!
//! - [`crate::Ncp`] implements `apis_saltans_hw::NcpDriver` when the
//!   wrapped transport implements the required EZSP command traits.
//! - [`crate::Builder`] implements `apis_saltans_hw::Start` and performs
//!   EZSP stack setup before spawning an `NcpDriver` actor.
//! - EZSP callbacks are translated into `apis_saltans_hw::Event` values.

mod conversion;
mod error;
mod ncp;
