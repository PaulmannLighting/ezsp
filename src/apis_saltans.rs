//! Integration with the `apis-saltans` Zigbee hardware traits.
//!
//! Enabling the `apis-saltans` feature adapts [`crate::Ncp`] to the
//! `apis_saltans_hw` driver API and adds custom startup helpers to
//! [`crate::Builder`]:
//!
//! - [`crate::Ncp`] implements `apis_saltans_hw::Driver` when the
//!   wrapped transport implements the required EZSP command traits.
//! - [`crate::Builder`] performs EZSP stack setup before spawning a `Driver`
//!   actor. Endpoint simple descriptors supplied to startup are registered with
//!   the NCP and kept as local cluster metadata for outgoing APS source
//!   endpoint selection.
//! - EZSP callbacks are translated into `apis_saltans_hw::Event` values.

mod conversion;
mod error;
mod ncp;
