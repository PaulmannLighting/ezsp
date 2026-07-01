use apis_saltans_hw::Event;

use crate::ember::Status;
use crate::ember::device::Update;
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

impl From<ChildJoin> for Event {
    fn from(child_join: ChildJoin) -> Self {
        if child_join.joining() {
            Self::DeviceJoined(child_join.into())
        } else {
            Self::DeviceLeft(child_join.into())
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
            Update::StandardSecurityUnsecuredJoin => Self::DeviceJoined(trust_center_join.into()),
            Update::StandardSecurityUnsecuredRejoin => Self::DeviceRejoined {
                address: trust_center_join.into(),
                secured: false,
            },
            Update::StandardSecuritySecuredRejoin => Self::DeviceRejoined {
                address: trust_center_join.into(),
                secured: true,
            },
            Update::DeviceLeft => Self::DeviceLeft(trust_center_join.into()),
        })
    }
}
