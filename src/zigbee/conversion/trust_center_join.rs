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
            Update::StandardSecurityUnsecuredJoin => Self::DeviceJoined {
                ieee_address: trust_center_join.new_node_eui64(),
                short_id: trust_center_join.new_node_id(),
            },
            Update::StandardSecurityUnsecuredRejoin => Self::DeviceRejoined {
                ieee_address: trust_center_join.new_node_eui64(),
                short_id: trust_center_join.new_node_id(),
                secured: false,
            },
            Update::StandardSecuritySecuredRejoin => Self::DeviceRejoined {
                ieee_address: trust_center_join.new_node_eui64(),
                short_id: trust_center_join.new_node_id(),
                secured: true,
            },
            Update::DeviceLeft => Self::DeviceLeft {
                ieee_address: trust_center_join.new_node_eui64(),
                short_id: trust_center_join.new_node_id(),
            },
        })
    }
}
