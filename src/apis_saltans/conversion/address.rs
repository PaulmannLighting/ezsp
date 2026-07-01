use apis_saltans_core::Address;

use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;

impl From<ChildJoin> for Address {
    fn from(child_join: ChildJoin) -> Self {
        Self::new(child_join.child_eui64(), child_join.child_id())
    }
}

impl From<TrustCenterJoin> for Address {
    fn from(trust_center_join: TrustCenterJoin) -> Self {
        Self::new(
            trust_center_join.new_node_eui64(),
            trust_center_join.new_node_id(),
        )
    }
}
