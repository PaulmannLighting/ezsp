//! APS data confirmations, membership, and network-state event conversions.
//!
//! Acknowledged direct-unicast `messageSent` callbacks recover the coordinator
//! correlation counter from the EZSP message tag and become APSDE data
//! confirmations. Child callbacks become join or leave events. Trust-center
//! callbacks distinguish unsecured joins, secured/unsecured rejoins, and
//! leaves. Only network up/down/opened/closed stack statuses have hardware
//! event variants.

use apis_saltans_hw::aps::apsde::{
    ConfirmStatus, DataConfirm, Destination, IndividualEndpoint, NetworkAddress,
};
use apis_saltans_hw::core::Endpoint;
use apis_saltans_hw::{ApsdeEvent, DeviceEvent, Event, NetworkEvent};

use crate::ember::Status;
use crate::ember::aps::Options;
use crate::ember::device::Update;
use crate::ember::message::Outgoing;
use crate::parameters::messaging::handler::MessageSent;
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

impl TryFrom<MessageSent> for Event {
    type Error = MessageSent;

    fn try_from(message_sent: MessageSent) -> Result<Self, Self::Error> {
        if !message_sent.aps_frame().options().contains(Options::RETRY) {
            return Err(message_sent);
        }
        let source_endpoint = Endpoint::try_from(message_sent.aps_frame().source_endpoint())
            .ok()
            .and_then(IndividualEndpoint::new)
            .ok_or_else(|| message_sent.clone())?;
        let destination_endpoint =
            Endpoint::try_from(message_sent.aps_frame().destination_endpoint())
                .map_err(|_| message_sent.clone())?;
        let destination = match message_sent.typ().map_err(|_| message_sent.clone())? {
            Outgoing::Direct => Destination::Network {
                address: NetworkAddress::new(message_sent.index_or_destination())
                    .ok_or_else(|| message_sent.clone())?,
                endpoint: destination_endpoint,
            },
            Outgoing::ViaAddressTable
            | Outgoing::ViaBinding
            | Outgoing::Multicast
            | Outgoing::Broadcast => return Err(message_sent),
        };
        let status = match message_sent.status() {
            Ok(Status::Success) => ConfirmStatus::success(),
            Ok(status) => ConfirmStatus::Network(status.into()),
            Err(status) => ConfirmStatus::Network(status),
        };
        let confirmation = DataConfirm::new(destination, source_endpoint, status, ());

        Ok(Self::Apsde(ApsdeEvent::DataConfirm {
            counter: message_sent.message_tag(),
            confirmation,
        }))
    }
}

impl TryFrom<ChildJoin> for Event {
    type Error = ChildJoin;

    fn try_from(child_join: ChildJoin) -> Result<Self, Self::Error> {
        let event = if child_join.joining() {
            DeviceEvent::Joined(child_join.try_into()?)
        } else {
            DeviceEvent::Left(child_join.try_into()?)
        };

        Ok(Self::Device(event))
    }
}

impl TryFrom<Status> for Event {
    type Error = Status;

    fn try_from(status: Status) -> Result<Self, Self::Error> {
        let event = match status {
            Status::NetworkUp => NetworkEvent::Up,
            Status::NetworkDown => NetworkEvent::Down,
            Status::NetworkOpened => NetworkEvent::Opened,
            Status::NetworkClosed => NetworkEvent::Closed,
            other => return Err(other),
        };

        Ok(Self::Network(event))
    }
}

impl TryFrom<TrustCenterJoin> for Event {
    type Error = TrustCenterJoin;

    fn try_from(trust_center_join: TrustCenterJoin) -> Result<Self, Self::Error> {
        let Ok(status) = trust_center_join.status() else {
            return Err(trust_center_join);
        };

        let event = match status {
            Update::StandardSecurityUnsecuredJoin => {
                DeviceEvent::Joined(trust_center_join.try_into()?)
            }
            Update::StandardSecurityUnsecuredRejoin => DeviceEvent::Rejoined {
                address: trust_center_join.try_into()?,
                secured: false,
            },
            Update::StandardSecuritySecuredRejoin => DeviceEvent::Rejoined {
                address: trust_center_join.try_into()?,
                secured: true,
            },
            Update::DeviceLeft => DeviceEvent::Left(trust_center_join.try_into()?),
        };

        Ok(Self::Device(event))
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::aps::apsde::{ConfirmStatus, Destination};
    use apis_saltans_hw::{ApsdeEvent, Event};
    use le_stream::FromLeStream;

    use crate::parameters::messaging::handler::MessageSent;

    const MESSAGE_TAG: u8 = 0x34;
    const APS_SEQUENCE: u8 = 0x56;
    const OPTIONS_INDEX: usize = 9;
    const STATUS_INDEX: usize = 15;
    const STATUS_SUCCESS: u8 = 0x00;
    const STATUS_DELIVERY_FAILED: u8 = 0x66;
    const MESSAGE_SENT_BYTES: [u8; 17] = [
        0x00,
        0x78,
        0x56,
        0x04,
        0x01,
        0x06,
        0x03,
        0x01,
        0x02,
        0x40,
        0x00,
        0x00,
        0x00,
        APS_SEQUENCE,
        MESSAGE_TAG,
        STATUS_SUCCESS,
        0x00,
    ];

    fn message_sent(status: u8) -> MessageSent {
        let mut bytes = MESSAGE_SENT_BYTES;
        bytes[STATUS_INDEX] = status;
        MessageSent::from_le_stream(bytes.into_iter())
            .expect("messageSent test callback is complete")
    }

    #[test]
    fn converts_successful_message_sent_to_data_confirmation() {
        let event = Event::try_from(message_sent(STATUS_SUCCESS))
            .expect("direct messageSent callback is representable");
        let Event::Apsde(ApsdeEvent::DataConfirm {
            counter,
            confirmation,
        }) = event
        else {
            panic!("messageSent must become a data confirmation");
        };

        assert_eq!(counter, MESSAGE_TAG);
        assert_eq!(confirmation.status(), ConfirmStatus::success());
        assert!(matches!(
            confirmation.destination(),
            Destination::Network { .. }
        ));
    }

    #[test]
    fn preserves_failed_message_sent_status() {
        let event = Event::try_from(message_sent(STATUS_DELIVERY_FAILED))
            .expect("direct messageSent callback is representable");
        let Event::Apsde(ApsdeEvent::DataConfirm { confirmation, .. }) = event else {
            panic!("messageSent must become a data confirmation");
        };

        assert_eq!(
            confirmation.status(),
            ConfirmStatus::Network(STATUS_DELIVERY_FAILED)
        );
    }

    #[test]
    fn rejects_unacknowledged_message_sent() {
        let mut message = MESSAGE_SENT_BYTES;
        message[OPTIONS_INDEX] = 0;
        let message = MessageSent::from_le_stream(message.into_iter())
            .expect("messageSent test callback is complete");

        assert!(Event::try_from(message).is_err());
    }
}
