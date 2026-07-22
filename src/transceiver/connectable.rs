use std::num::NonZero;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;

use crate::transceiver::{Connection, Message};
use crate::{Callback, Error};

/// Handle to running transport tasks before EZSP version negotiation.
///
/// Call [`Connectable::connect`] to issue the initial `version` command. On a
/// transport whose actors are returned as futures, the caller must spawn those
/// actors before calling `connect`.
#[derive(Debug)]
pub struct Connectable {
    pub(crate) handle: Sender<Message>,
    pub(crate) callbacks: Receiver<Callback>,
}

impl Connectable {
    /// Negotiates `desired_version` and returns a connected handle and callback stream.
    ///
    /// The returned [`Connection`] value is cloneable and implements the typed
    /// command traits through [`Communicate`](crate::Communicate).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor channel closes, transmission fails, or
    /// the NCP negotiates a different protocol version.
    pub async fn connect(
        self,
        desired_version: NonZero<u8>,
    ) -> Result<(Connection, Receiver<Callback>), Error> {
        let (response, rx) = oneshot::channel();

        self.handle
            .send(Message::Connect {
                desired_version,
                response,
            })
            .await?;

        rx.await??;

        Ok((
            Connection {
                handle: self.handle,
            },
            self.callbacks,
        ))
    }
}
