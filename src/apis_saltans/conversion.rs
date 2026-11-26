//! Conversion implementations between EZSP and `apis-saltans` data models.
//!
//! The driver uses these conversions for endpoints, scan results, and outgoing
//! APS transmission options. The event path uses them for device addresses,
//! membership/network callbacks, APSDE indications, and data confirmations.
//!
//! `TryFrom<Callback> for apis_saltans_hw::Event` recognizes `messageSent`,
//! child additions, successful stack-status, and trust-center-join callbacks.
//! Child removals, unsupported callback families, unrecognized Ember statuses,
//! raw status errors, and trust-center join or rejoin attempts without an
//! accepted policy decision fail conversion. The high-level NCP event handler
//! consumes fragment-internal `messageSent` callbacks before conversion.
//!
//! Incoming-message conversion is deliberately separate: a
//! [`DefragmentedMessage`] converts into an
//! `apis_saltans_hw::aps::apsde::DataIndication` or an
//! `apis_saltans_hw::Event::Apsde` receive event.

use apis_saltans_hw::aps::apsde::DataIndication;
use apis_saltans_hw::{ApsdeEvent, Event};
use bytes::Bytes;

pub use self::error::ParseApsFrameError;
use crate::ember::aps::Options;
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
        let zdo_response_required = defragmented_message
            .aps_frame()
            .options()
            .contains(Options::ZDO_RESPONSE_REQUIRED);

        DataIndication::<Bytes, ()>::try_from(defragmented_message)
            .map(|indication| ApsdeEvent::DataIndication {
                indication,
                zdo_response_required,
            })
            .map(Self::Apsde)
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::{ApsdeEvent, DeviceEvent, Event};
    use le_stream::FromLeStream;

    use crate::ember::device::Update;
    use crate::ember::join::Decision;
    use crate::frame::{Parameter, Parsable};
    use crate::parameters::messaging::handler::IncomingMessage;
    use crate::parameters::networking::handler::ChildJoin;
    use crate::parameters::trust_center::handler::TrustCenterJoin;
    use crate::{Callback, DefragmentedMessage};

    const CHILD_JOINING_INDEX: usize = 1;
    const CHILD_JOIN_BYTES: [u8; 13] = [
        0x00, 0x00, 0x34, 0x12, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x03,
    ];
    const TRUST_CENTER_STATUS_INDEX: usize = 10;
    const TRUST_CENTER_DECISION_INDEX: usize = 11;
    const UNKNOWN_TRUST_CENTER_DECISION: u8 = 0xff;
    const TRUST_CENTER_JOIN_BYTES: [u8; 14] = [
        0x34, 0x12, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x03, 0x00, 0x00,
    ];
    const ZDO_RESPONSE_REQUIRED_INDEX: usize = 8;
    const ZDO_RESPONSE_REQUIRED: u8 = 0x40;
    const INCOMING_MESSAGE_BYTES: [u8; 20] = [
        0x00, 0x04, 0x01, 0x06, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x56, 0x80, 0xd8, 0x34,
        0x12, 0xff, 0xff, 0x01, 0xaa,
    ];

    fn child_join_callback(joining: bool) -> Callback {
        let mut bytes = CHILD_JOIN_BYTES;
        bytes[CHILD_JOINING_INDEX] = u8::from(joining);

        Callback::parse_from_le_stream(ChildJoin::ID, bytes.into_iter())
            .expect("childJoinHandler test callback is complete")
    }

    #[test]
    fn child_addition_emits_device_joined() {
        assert!(matches!(
            Event::try_from(child_join_callback(true)),
            Ok(Event::Device(DeviceEvent::Joined(_)))
        ));
    }

    #[test]
    fn child_removal_emits_no_event() {
        assert!(Event::try_from(child_join_callback(false)).is_err());
    }

    fn trust_center_callback(status: Update, decision: u8) -> Callback {
        let mut bytes = TRUST_CENTER_JOIN_BYTES;
        bytes[TRUST_CENTER_STATUS_INDEX] = status.into();
        bytes[TRUST_CENTER_DECISION_INDEX] = decision;

        Callback::parse_from_le_stream(TrustCenterJoin::ID, bytes.into_iter())
            .expect("trust center test callback is complete")
    }

    #[test]
    fn trust_center_denied_or_unknown_join_decision_emits_no_event() {
        for status in [
            Update::StandardSecurityUnsecuredJoin,
            Update::StandardSecurityUnsecuredRejoin,
            Update::StandardSecuritySecuredRejoin,
        ] {
            for decision in [Decision::DenyJoin.into(), UNKNOWN_TRUST_CENTER_DECISION] {
                assert!(
                    Event::try_from(trust_center_callback(status, decision)).is_err(),
                    "unexpected event for {status:?} with policy decision {decision:#04x}"
                );
            }
        }
    }

    #[test]
    fn trust_center_no_action_on_unsecured_attempt_emits_no_event() {
        for status in [
            Update::StandardSecurityUnsecuredJoin,
            Update::StandardSecurityUnsecuredRejoin,
        ] {
            assert!(
                Event::try_from(trust_center_callback(status, Decision::NoAction.into())).is_err(),
                "unexpected event for {status:?} with NoAction"
            );
        }
    }

    #[test]
    fn trust_center_allowed_unsecured_attempt_emits_join_or_rejoin() {
        for decision in [Decision::UsePreconfiguredKey, Decision::SendKeyInTheClear] {
            assert!(matches!(
                Event::try_from(trust_center_callback(
                    Update::StandardSecurityUnsecuredJoin,
                    decision.into(),
                )),
                Ok(Event::Device(DeviceEvent::Joined(_)))
            ));
            assert!(matches!(
                Event::try_from(trust_center_callback(
                    Update::StandardSecurityUnsecuredRejoin,
                    decision.into(),
                )),
                Ok(Event::Device(DeviceEvent::Rejoined { secured: false, .. }))
            ));
        }
    }

    #[test]
    fn trust_center_allowed_secured_rejoin_emits_secured_rejoin() {
        for decision in [
            Decision::NoAction,
            Decision::UsePreconfiguredKey,
            Decision::SendKeyInTheClear,
        ] {
            assert!(matches!(
                Event::try_from(trust_center_callback(
                    Update::StandardSecuritySecuredRejoin,
                    decision.into(),
                )),
                Ok(Event::Device(DeviceEvent::Rejoined { secured: true, .. }))
            ));
        }
    }

    #[test]
    fn trust_center_device_left_emits_device_left() {
        for decision in [
            Decision::NoAction.into(),
            Decision::DenyJoin.into(),
            UNKNOWN_TRUST_CENTER_DECISION,
        ] {
            assert!(matches!(
                Event::try_from(trust_center_callback(Update::DeviceLeft, decision)),
                Ok(Event::Device(DeviceEvent::Left(_)))
            ));
        }
    }

    fn incoming_event(zdo_response_required: bool) -> Event {
        let mut bytes = INCOMING_MESSAGE_BYTES;
        if zdo_response_required {
            bytes[ZDO_RESPONSE_REQUIRED_INDEX] = ZDO_RESPONSE_REQUIRED;
        }
        let incoming_message = IncomingMessage::from_le_stream(bytes.into_iter())
            .expect("incomingMessage test callback is complete");

        Event::try_from(DefragmentedMessage::from(incoming_message))
            .expect("incomingMessage test callback is representable")
    }

    #[test]
    fn preserves_zdo_response_required_flag() {
        for expected in [false, true] {
            let Event::Apsde(ApsdeEvent::DataIndication {
                zdo_response_required,
                ..
            }) = incoming_event(expected)
            else {
                panic!("incomingMessage must become a data indication");
            };

            assert_eq!(zdo_response_required, expected);
        }
    }
}
