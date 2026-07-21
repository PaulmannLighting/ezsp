//! Membership and network-state event conversions.
//!
//! Child callbacks become join or leave events. Trust-center callbacks
//! distinguish unsecured joins, secured/unsecured rejoins, and leaves. Only
//! network up/down/opened/closed stack statuses have hardware event variants.

use apis_saltans_hw::Event;

use crate::ember::Status;
use crate::ember::device::Update;
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

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
