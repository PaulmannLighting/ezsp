use tokio::sync::mpsc::Receiver;

use crate::Callback;
use crate::ember::Status;
use crate::frame::parameters::networking::handler::Handler as Networking;

pub trait AwaitEvent {
    fn await_network_status(&mut self, status: Status) -> impl Future<Output = ()> + Send;

    fn await_network_up(&mut self) -> impl Future<Output = ()> + Send {
        self.await_network_status(Status::NetworkUp)
    }

    fn await_network_down(&mut self) -> impl Future<Output = ()> + Send {
        self.await_network_status(Status::NetworkDown)
    }
}

impl AwaitEvent for Receiver<Callback> {
    async fn await_network_status(&mut self, status: Status) {
        while let Some(callback) = self.recv().await {
            if let Callback::Networking(Networking::StackStatus(stack_status)) = callback
                && stack_status.result() == Ok(status)
            {
                return;
            }
        }
    }
}
