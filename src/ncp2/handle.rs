use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot;

use super::Message;
use crate::ember::NodeId;
use crate::ember::aps::Frame;
use crate::ember::message::Destination;
use crate::frame::parameters::messaging::{send_broadcast, send_multicast, send_unicast};
use crate::types::ByteSizedVec;
use crate::{Error, Parameters};

/// A cloneable handle for sending requests to an actor-based NCP.
#[derive(Clone, Debug)]
pub struct Handle {
    sender: Sender<Message>,
}

impl Handle {
    /// Wraps an NCP actor message sender.
    #[must_use]
    pub const fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    /// Forwards an incoming EZSP frame to the NCP actor.
    ///
    /// # Errors
    ///
    /// Returns [`SendError`] when the actor has stopped.
    pub async fn received(&self, parameters: Parameters) -> Result<(), SendError<Message>> {
        self.sender.send(Message::Received(parameters)).await
    }

    /// Sends a unicast APS message and returns its NCP-assigned sequence.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor has stopped, transmission fails, the
    /// response channel closes, or the NCP rejects the command.
    pub async fn unicast(
        &self,
        destination: Destination,
        aps_frame: Frame,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let command = send_unicast::Command::new(destination, aps_frame, message_tag, message);
        let (response, receiver) = oneshot::channel();
        self.sender
            .send(Message::Unicast {
                command: Box::new(command),
                response,
            })
            .await?;
        receiver.await?
    }

    /// Sends a multicast APS message and returns its NCP-assigned sequence.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor has stopped, transmission fails, the
    /// response channel closes, or the NCP rejects the command.
    pub async fn multicast(
        &self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let command =
            send_multicast::Command::new(aps_frame, hops, nonmember_radius, message_tag, message);
        let (response, receiver) = oneshot::channel();
        self.sender
            .send(Message::Multicast {
                command: Box::new(command),
                response,
            })
            .await?;
        receiver.await?
    }

    /// Sends a broadcast APS message and returns its NCP-assigned sequence.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor has stopped, transmission fails, the
    /// response channel closes, or the NCP rejects the command.
    pub async fn broadcast(
        &self,
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        let command =
            send_broadcast::Command::new(destination, aps_frame, radius, message_tag, message);
        let (response, receiver) = oneshot::channel();
        self.sender
            .send(Message::Broadcast {
                command: Box::new(command),
                response,
            })
            .await?;
        receiver.await?
    }
}

impl From<Sender<Message>> for Handle {
    fn from(sender: Sender<Message>) -> Self {
        Self::new(sender)
    }
}
