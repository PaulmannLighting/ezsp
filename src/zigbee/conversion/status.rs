use zigbee_hw::Event;

use crate::ember::Status;

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
