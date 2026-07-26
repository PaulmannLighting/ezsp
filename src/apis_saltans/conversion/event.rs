//! Membership and network-state event conversions.
//!
//! `messageSent` callbacks recover the application APS sequence from the EZSP
//! message tag and become acknowledgement events. Child callbacks become join
//! or leave events. Trust-center callbacks distinguish unsecured joins,
//! secured/unsecured rejoins, and leaves. Only network
//! up/down/opened/closed stack statuses have hardware event variants.

use apis_saltans_hw::Event;

use crate::ember::Status;
use crate::ember::device::Update;
use crate::parameters::messaging::handler::MessageSent;
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

impl From<MessageSent> for Event {
    fn from(message_sent: MessageSent) -> Self {
        let sequence = message_sent.message_tag();

        match message_sent.status() {
            Ok(Status::Success) => Self::Ack(sequence),
            status => Self::Nak {
                sequence,
                error: crate::Error::from(status).into(),
            },
        }
    }
}

impl TryFrom<ChildJoin> for Event {
    type Error = ChildJoin;

    fn try_from(child_join: ChildJoin) -> Result<Self, Self::Error> {
        if child_join.joining() {
            Ok(Self::DeviceJoined(child_join.try_into()?))
        } else {
            Ok(Self::DeviceLeft(child_join.try_into()?))
        }
    }
}

impl TryFrom<Status> for Event {
    type Error = Status;

    fn try_from(status: Status) -> Result<Self, Self::Error> {
        match status {
            Status::NetworkUp => Ok(Self::NetworkUp),
            Status::NetworkDown => Ok(Self::NetworkDown),
            Status::NetworkOpened => Ok(Self::NetworkOpened),
            Status::NetworkClosed => Ok(Self::NetworkClosed),
            other => Err(other),
        }
    }
}

impl TryFrom<TrustCenterJoin> for Event {
    type Error = TrustCenterJoin;

    fn try_from(trust_center_join: TrustCenterJoin) -> Result<Self, Self::Error> {
        let Ok(status) = trust_center_join.status() else {
            return Err(trust_center_join);
        };

        Ok(match status {
            Update::StandardSecurityUnsecuredJoin => {
                Self::DeviceJoined(trust_center_join.try_into()?)
            }
            Update::StandardSecurityUnsecuredRejoin => Self::DeviceRejoined {
                address: trust_center_join.try_into()?,
                secured: false,
            },
            Update::StandardSecuritySecuredRejoin => Self::DeviceRejoined {
                address: trust_center_join.try_into()?,
                secured: true,
            },
            Update::DeviceLeft => Self::DeviceLeft(trust_center_join.try_into()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use apis_saltans_hw::Event;
    use le_stream::FromLeStream;

    use crate::parameters::messaging::handler::MessageSent;

    const MESSAGE_TAG: u8 = 0x34;
    const APS_SEQUENCE: u8 = 0x56;
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
        0x00,
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
    fn converts_successful_message_sent_to_ack() {
        assert!(matches!(
            Event::from(message_sent(STATUS_SUCCESS)),
            Event::Ack(MESSAGE_TAG)
        ));
    }

    #[test]
    fn converts_failed_message_sent_to_nak() {
        assert!(matches!(
            Event::from(message_sent(STATUS_DELIVERY_FAILED)),
            Event::Nak {
                sequence: MESSAGE_TAG,
                ..
            }
        ));
    }
}
