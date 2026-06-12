use zigbee_hw::Event;

use crate::ember::device::Update;
use crate::parameters::trust_center::handler::TrustCenterJoin;

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
