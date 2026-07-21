use tokio::spawn;
use tokio::sync::mpsc::channel;

pub use self::connected::Connected;
pub use self::disconnected::Disconnected;
pub use self::message::Message;
pub use self::receiver::{Receive, Receiver};
pub use self::translatable_event::TranslatableEvent;
pub use self::transmitter::{Transmit, Transmitter};

mod callbacks;
mod connected;
mod disconnected;
mod message;
mod receiver;
mod translatable_event;
mod transmitter;

pub struct Transceiver<T, R> {
    transmit: T,
    receive: R,
}

impl<T, R> Transceiver<T, R> {
    #[must_use]
    pub const fn new(transmit: T, receive: R) -> Self {
        Self { transmit, receive }
    }
}

impl<T, R> Transceiver<T, R>
where
    T: Transmit + Send + 'static,
    R: Receive + Send + 'static,
{
    pub fn spawn(self, callbacks_capacity: usize, messages_capacity: usize) -> Disconnected {
        let (msg_tx, msg_rx) = channel(messages_capacity);
        let (cb_tx, cb_rx) = channel(callbacks_capacity);
        let tx_task = spawn(Transmitter::new(self.transmit, msg_rx).run());
        let rx_task = spawn(Receiver::new(self.receive, cb_tx, msg_tx.clone()).run());
        Disconnected {
            handle: msg_tx,
            callbacks: cb_rx,
            tx_task,
            rx_task,
        }
    }
}
