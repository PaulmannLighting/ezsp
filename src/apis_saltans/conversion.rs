//! Conversion implementations between EZSP and `apis-saltans` data models.
//!
//! The driver uses these conversions for endpoints, scan results, and outgoing
//! APS transmission options. The event path uses them for device addresses,
//! membership/network callbacks, APSDE indications, and data confirmations.
//!
//! `TryFrom<Callback> for apis_saltans_hw::Event` recognizes `messageSent`,
//! child-join, successful stack-status, and trust-center-join callbacks.
//! Unsupported callback families, unrecognized Ember statuses, and raw status
//! errors return `Err(())`. Fragment-internal `messageSent` callbacks are
//! consumed by the high-level NCP event handler before this conversion is
//! attempted.
//!
//! Incoming-message conversion is deliberately separate: a
//! [`DefragmentedMessage`] converts into an
//! `apis_saltans_hw::aps::apsde::DataIndication` or an
//! `apis_saltans_hw::Event::Apsde` receive event.

use apis_saltans_hw::aps::apsde::DataIndication;
use apis_saltans_hw::{ApsdeEvent, Event};
use bytes::Bytes;

pub use self::error::ParseApsFrameError;
use crate::frame::parameters::networking::handler::Handler as Networking;
use crate::parameters::messaging::handler::Handler as Messaging;
use crate::parameters::trust_center::handler::Handler as TrustCenter;
use crate::{Callback, DefragmentedMessage};

mod address;
mod aps_options;
mod defragmented_message;
mod endpoint;
mod error;
mod event;
mod found_network;
mod scanned_channel;

const UNHANDLED_EVENT: &str = "Unhandled event.";

impl TryFrom<Callback> for Event {
    type Error = &'static str;

    fn try_from(callback: Callback) -> Result<Self, Self::Error> {
        match callback {
            Callback::Messaging(Messaging::MessageSent(message_sent)) => {
                return Self::try_from(*message_sent).map_err(|_| UNHANDLED_EVENT);
            }
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
    type Error = <DataIndication<Bytes, ()> as TryFrom<DefragmentedMessage>>::Error;

    fn try_from(defragmented_message: DefragmentedMessage) -> Result<Self, Self::Error> {
        DataIndication::<Bytes, ()>::try_from(defragmented_message)
            .map(ApsdeEvent::DataIndication)
            .map(Self::Apsde)
    }
}
