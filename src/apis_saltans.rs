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
//! datagram transmission to typed EZSP commands and high-level NCP workflows.
//!
//! [`crate::Builder::start`] returns an [`Ncp`](crate::Ncp) inside its build
//! result. To access it through an `apis-saltans` actor handle, call
//! `apis_saltans_hw::Driver::run` on that value and spawn the returned future.
//! The `apis-saltans` driver actor serializes hardware requests; it is separate
//! from the internal EZSP transmitter and receiver actors.
//!
//! # Endpoints
//!
//! `apis_saltans_hw::zdp::SimpleDescriptor` converts into [`crate::Endpoint`]
//! for [`crate::Builder::start`]. The reverse conversion is used by
//! `Driver::get_endpoints`. A reverse conversion can fail when an `Endpoint`
//! contains an unsupported profile ID or reserved endpoint number; the driver
//! logs and omits such a descriptor.
//!
//! # Events and incoming messages
//!
//! EZSP child, trust-center, and stack-status callbacks convert into
//! `apis_saltans_hw::Event` membership and network-state events. Incoming APS
//! messages have separate conversions into `apis_saltans_hw::aps::Data` and a
//! NWK `Envelope`, preserving APS addressing, payload, link quality, RSSI,
//! binding index, and source-route overhead.
//!
//! The feature does not currently implement
//! `TryFrom<crate::DefragmentedMessage>` directly for
//! `apis_saltans_hw::Event`. Consequently, that event enum by itself does not
//! satisfy [`crate::TranslatableEvent`] for [`crate::Builder::start`]; an
//! application event wrapper must supply both required conversions.
//!
//! # Errors
//!
//! Crate [`crate::Error`] values convert to
//! `apis_saltans_hw::Error::Implementation` behind an [`std::sync::Arc`]. Errors
//! that occur after an APS send command is accepted are reported when the
//! returned `apis_saltans_hw::HwResponse` is awaited.

mod conversion;
mod error;
mod ncp;
