use zigbee_hw::Event;

use crate::ember::Status;

impl TryFrom<Status> for Event {
    type Error = Status;

    fn try_from(status: Status) -> Result<Self, Self::Error> {
        match status {
            Status::NetworkUp => Ok(Event::NetworkUp),
            Status::NetworkDown => Ok(Event::NetworkDown),
            Status::NetworkOpened => Ok(Event::NetworkOpened),
            Status::NetworkClosed => Ok(Event::NetworkClosed),
            other => Err(other),
        }
    }
}
