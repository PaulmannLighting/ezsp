use crate::ember::message::Destination;

pub trait SendUnicast {
    fn send_unicast(&self, typ: Destination, payload: &[u8]) -> Result<(), String>;
}
