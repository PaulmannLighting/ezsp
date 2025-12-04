use tokio::sync::mpsc::Receiver;

use crate::Callback;
use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as NetworkingEvent;

/// Trait for checking the stack status.
pub trait StackStatus {
    /// Return the current stack status as a string.
    fn stack_status(&mut self, target_status: Status) -> impl Future<Output = ()>;
}

impl StackStatus for Receiver<Callback> {
    async fn stack_status(&mut self, target_status: Status) {
        while let Some(event) = self.recv().await {
            if let Callback::Networking(NetworkingEvent::StackStatus(status)) = event
                && status.result() == Ok(target_status)
            {
                return;
            }
        }
    }
}
