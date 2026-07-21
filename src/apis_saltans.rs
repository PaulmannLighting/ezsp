//! Integration with the `apis-saltans` Zigbee hardware traits.
//!
//! Enabling the `apis-saltans` feature adds a newtype driver adapter around
//! [`crate::Ncp`] and custom startup helpers to [`crate::Builder`]:
//!
//! - The internal `ZigbeeNcp` adapter implements `apis_saltans_hw::Driver` when
//!   the wrapped transport implements the required EZSP command traits. The
//!   plain [`crate::Ncp`] remains the transport-independent host helper.
//! - [`crate::Builder`] performs EZSP stack setup and constructs the adapter
//!   before spawning a `Driver` actor. Adapter construction registers every
//!   endpoint simple descriptor supplied to startup with the device and keeps
//!   the descriptors available to the driver. Their output clusters are also
//!   retained by the wrapped [`crate::Ncp`] for APS source-endpoint selection.
//! - EZSP callbacks are translated into `apis_saltans_hw::Event` values.

mod conversion;
mod error;
mod ncp;
