use apis_saltans_hw::core::FullAddress;
use apis_saltans_hw::core::short_id::Device;

use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

impl TryFrom<ChildJoin> for FullAddress {
    type Error = ChildJoin;

    fn try_from(child_join: ChildJoin) -> Result<Self, Self::Error> {
        let Some(short_id) = Device::new(child_join.child_id()) else {
            return Err(child_join);
        };

        Ok(Self::new(child_join.child_eui64().into(), short_id))
    }
}

impl TryFrom<TrustCenterJoin> for FullAddress {
    type Error = TrustCenterJoin;

    fn try_from(trust_center_join: TrustCenterJoin) -> Result<Self, Self::Error> {
        let Some(short_id) = Device::new(trust_center_join.new_node_id()) else {
            return Err(trust_center_join);
        };

        Ok(Self::new(
            trust_center_join.new_node_eui64().into(),
            short_id,
        ))
    }
}
