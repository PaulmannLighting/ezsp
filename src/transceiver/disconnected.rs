use std::num::NonZero;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::transceiver::{Connected, Message};
use crate::{Callback, Error};

/// Handle to running transport tasks before EZSP version negotiation.
///
/// Call [`Disconnected::connect`] to issue the initial `version` command. On a
/// negotiation error both transport tasks are aborted.
#[derive(Debug)]
pub struct Disconnected {
    pub(crate) handle: Sender<Message>,
    pub(crate) callbacks: Receiver<Callback>,
    pub(crate) tx_task: JoinHandle<()>,
    pub(crate) rx_task: JoinHandle<()>,
}

impl Disconnected {
    /// Negotiates `desired_version` and returns a connected handle and callback stream.
    ///
    /// The returned [`Connected`] value is cloneable and implements the typed
    /// command traits through [`Communicate`](crate::Communicate).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor channel closes, transmission fails, or
    /// the NCP negotiates a different protocol version. On error, the spawned
    /// transmitter and receiver tasks are aborted.
    pub async fn connect(
        self,
        desired_version: NonZero<u8>,
    ) -> Result<(Connected, Receiver<Callback>), Error> {
        match Self::connect_inner(self.handle, desired_version).await {
            Ok(handle) => Ok((Connected { handle }, self.callbacks)),
            Err(error) => {
                self.tx_task.abort();
                self.rx_task.abort();
                Err(error)
            }
        }
    }

    async fn connect_inner(
        handle: Sender<Message>,
        desired_version: NonZero<u8>,
    ) -> Result<Sender<Message>, Error> {
        let (response, rx) = oneshot::channel();

        handle
            .send(Message::Connect {
                desired_version,
                response,
            })
            .await?;

        rx.await??;

        Ok(handle)
    }
}
