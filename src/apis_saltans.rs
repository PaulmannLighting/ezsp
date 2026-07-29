//! Integration with the `apis-saltans` Zigbee hardware abstraction.
//!
//! Enabling the `apis-saltans` feature adds trait implementations and data-model
//! conversions; it does not introduce another EZSP transport or wrap
//! [`crate::Ncp`] in an adapter type.
//!
//! # Driver implementation
//!
//! [`crate::Ncp`] implements `apis_saltans_hw::Driver`. The implementation maps
//! identity lookup, scans, permit-joining, route discovery, address lookup, and
//! APSDE data requests to typed EZSP commands and high-level NCP workflows.
//!
//! [`crate::Builder::start`] returns an [`Ncp`](crate::Ncp) inside its build
//! result. To access it through an `apis-saltans` actor handle, call
//! `apis_saltans_hw::Driver::into_actor` on that value with a nonzero channel
//! capacity, then spawn the returned future. The `apis-saltans` driver actor
//! serializes hardware requests; it is separate from the internal EZSP
//! transmitter and receiver actors.
//!
//! # Endpoints
//!
//! `apis_saltans_hw::zdp::SimpleDescriptor` converts into [`crate::Endpoint`]
//! for [`crate::Builder::start`]. The reverse conversion is used by
//! `Driver::get_endpoints`. A reverse conversion can fail when an `Endpoint`
//! contains an unsupported profile ID; the driver logs and omits such a
//! descriptor.
//!
//! # Events and incoming messages
//!
//! EZSP child, trust-center, stack-status, and final `messageSent` callbacks
//! convert into the corresponding `apis_saltans_hw::Event::Device`,
//! `apis_saltans_hw::Event::Network`, and `apis_saltans_hw::Event::Apsde`
//! categories. Incoming APS messages convert into
//! `apis_saltans_hw::aps::apsde::DataIndication` values, preserving APSDE
//! addressing, profile, cluster, payload, and link quality. EZSP provides
//! neither a reception timestamp nor a device-key-pair handle, so both
//! backend-defined context values are `()`.
//!
//! # Errors
//!
//! Crate [`crate::Error`] values convert to
//! `apis_saltans_hw::Error::Backend`, preserving the concrete error as its
//! source. Final acknowledged-unicast `messageSent` statuses are preserved in
//! `apis_saltans_hw::aps::apsde::DataConfirm` values.

mod conversion;
mod error;
mod ncp;
