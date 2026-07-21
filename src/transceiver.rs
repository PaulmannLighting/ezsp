//! Actor-based EZSP transport orchestration.
//!
//! A [`Transceiver`] combines independent [`Transmit`] and [`Receive`] halves.
//! Spawning it creates a transmitter actor and a receiver task connected by
//! bounded channels. The receiver routes response frames back to the actor and
//! asynchronous callbacks to a separate stream. After version negotiation, the
//! cloneable [`Connected`] handle provides typed command/response transactions.

use tokio::spawn;
use tokio::sync::mpsc::channel;

pub use self::connected::Connected;
pub use self::disconnected::Disconnected;
pub use self::message::Message;
pub use self::receiver::{Receive, Receiver};
pub use self::translatable_event::TranslatableEvent;
pub use self::transmitter::{Transmit, Transmitter};

mod connected;
mod disconnected;
mod message;
mod receiver;
mod translatable_event;
mod transmitter;

/// Combines the outbound and inbound halves of an EZSP transport.
///
/// This value is a configuration object until [`Transceiver::spawn`] starts the
/// actor tasks. Custom transports implement [`Transmit`] and [`Receive`]
/// independently and join them with this type.
pub struct Transceiver<T, R> {
    transmit: T,
    receive: R,
}

impl<T, R> Transceiver<T, R> {
    /// Creates a transceiver from independent transmit and receive halves.
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
    /// Spawns the transmitter actor and receiver task.
    ///
    /// `callbacks_capacity` bounds asynchronous callback buffering, while
    /// `messages_capacity` bounds commands and internally forwarded responses.
    /// The returned [`Disconnected`] handle must negotiate an EZSP version
    /// before commands can be issued.
    #[must_use]
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
