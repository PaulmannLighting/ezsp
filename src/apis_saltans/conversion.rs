//! Conversion implementations between EZSP and `apis-saltans` data models.
//!
//! The driver uses these conversions for endpoints and scan results. The event
//! path uses them for device addresses, membership/network callbacks, complete
//! incoming APS messages, NWK envelopes, and reception metadata.
//!
//! `TryFrom<Callback> for apis_saltans_hw::Event` recognizes only child-join,
//! successful stack-status, and trust-center-join callbacks. Unsupported
//! callback families, unrecognized Ember statuses, and raw status errors return
//! `Err(())`. Scan callbacks and `messageSent` callbacks are consumed by the
//! high-level NCP event handler before this conversion is attempted.
//!
//! Incoming-message conversion is deliberately separate: a
//! [`DefragmentedMessage`] converts into `apis_saltans_hw::aps::Data` or
//! a NWK envelope, but not directly into `apis_saltans_hw::Event`.

use apis_saltans_hw::Event;
use apis_saltans_hw::aps::Data;
use apis_saltans_hw::nwk::Envelope;
use bytes::Bytes;

pub use self::error::ParseApsFrameError;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::trust_center::handler::Handler as TrustCenter;
use crate::{Callback, DefragmentedMessage};

mod address;
mod defragmented_message;
mod endpoint;
mod envelope;
mod error;
mod event;
mod found_network;
mod metadata;
mod scanned_channel;

const UNHANDLED_EVENT: &str = "Unhandled event.";

impl TryFrom<Callback> for Event {
    type Error = &'static str;

    fn try_from(callback: Callback) -> Result<Self, Self::Error> {
        match callback {
            Callback::Networking(Networking::ChildJoin(child_join)) => {
                return Self::try_from(*child_join).map_err(|_| UNHANDLED_EVENT);
            }
            Callback::Networking(Networking::StackStatus(status)) => {
                if let Ok(status) = status.result() {
                    return Self::try_from(status).map_err(|_| UNHANDLED_EVENT);
                }
            }
            Callback::TrustCenter(TrustCenter::TrustCenterJoin(trust_center_join)) => {
                return Self::try_from(*trust_center_join).map_err(|_| UNHANDLED_EVENT);
            }
            _ => return Err(UNHANDLED_EVENT),
        }

        Err(UNHANDLED_EVENT)
    }
}

impl TryFrom<DefragmentedMessage> for Event {
    type Error = <Envelope<Data<Bytes>> as TryFrom<DefragmentedMessage>>::Error;

    fn try_from(defragmented_message: DefragmentedMessage) -> Result<Self, Self::Error> {
        Envelope::<Data<Bytes>>::try_from(defragmented_message).map(Self::MessageReceived)
    }
}
