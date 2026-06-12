use zigbee_hw::Event;

use crate::parameters::networking::handler::ChildJoin;

impl From<ChildJoin> for Event {
    fn from(child_join: ChildJoin) -> Self {
        if child_join.joining() {
            Self::DeviceJoined(child_join.into())
        } else {
            Self::DeviceLeft(child_join.into())
        }
    }
}
