use zigbee_hw::Event;

use crate::parameters::networking::handler::ChildJoin;

impl From<ChildJoin> for Event {
    fn from(child_join: ChildJoin) -> Self {
        if child_join.joining() {
            Event::DeviceJoined {
                ieee_address: child_join.child_eui64(),
                short_id: child_join.child_id(),
            }
        } else {
            Event::DeviceLeft {
                ieee_address: child_join.child_eui64(),
                short_id: child_join.child_id(),
            }
        }
    }
}
