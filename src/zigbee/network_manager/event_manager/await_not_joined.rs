use crate::Callback;
use crate::ember::Status;
use crate::parameters::networking::handler::Handler as NetworkingEvent;
use crate::zigbee::EventManager;

pub trait AwaitNotJoined {
    /// Wait until the network is up and running.
    async fn await_not_joined(&mut self, buffer: usize);
}

impl AwaitNotJoined for EventManager {
    async fn await_not_joined(&mut self, buffer: usize) {
        let mut event_handler = self.register(buffer).await;

        while let Some(event) = event_handler.recv().await {
            if let Callback::Networking(NetworkingEvent::StackStatus(status)) = event {
                if status.result() == Ok(Status::NotJoined) {
                    return;
                }
            }
        }
    }
}
