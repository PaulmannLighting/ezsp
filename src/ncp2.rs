//! Actor-based EZSP Network Co-Processor support.
//!
//! [`Ncp`] owns the transmitting half of an EZSP connection and processes
//! requests delivered through a [`Handle`]. Incoming frames are returned to
//! the actor as [`Message::Received`] messages. This keeps the command/response
//! state in one task without requiring a mutex around the transport.

use std::collections::VecDeque;

use log::{trace, warn};
use tokio::sync::mpsc::Receiver as MessageReceiver;

pub use self::handle::{Handle, Handle as NcpHandle};
pub use self::message::Message;
pub use self::receive::Receive;
pub use self::transmit::Transmit;
use crate::frame::parameters::messaging::{send_broadcast, send_multicast, send_unicast};
use crate::{Error, Parameters};

mod handle;
mod message;
mod receive;
mod transmit;

/// An actor that serializes EZSP commands over a transmitter.
///
/// Only one command is in flight at a time. Further requests remain queued
/// until the actor receives the response for the current command through
/// [`Message::Received`]. Asynchronous callback parameters do not complete the
/// in-flight command.
#[derive(Debug)]
pub struct Ncp<T>
where
    T: Transmit,
{
    transmitter: T,
}

impl<T> Ncp<T>
where
    T: Transmit,
{
    /// Creates an actor backed by `transmitter`.
    #[must_use]
    pub const fn new(transmitter: T) -> Self {
        Self { transmitter }
    }

    /// Processes actor messages until every [`Handle`] sender is dropped.
    ///
    /// Outgoing requests are serialized because EZSP command responses do not
    /// carry enough information at this layer to correlate multiple concurrent
    /// commands. A transport receive task should forward every incoming frame
    /// to the actor as [`Message::Received`].
    pub async fn run(mut self, mut receiver: MessageReceiver<Message>) {
        let mut pending: Option<Pending> = None;
        let mut queued = VecDeque::new();

        while let Some(message) = receiver.recv().await {
            match message {
                Message::Received(Parameters::Callback(callback)) => {
                    trace!("Received callback while running NCP actor: {callback:?}");
                }
                Message::Received(parameters) => {
                    if let Some(request) = pending.take() {
                        request.complete(parameters);
                    } else {
                        warn!(
                            "Received an EZSP response without a pending request: {parameters:?}"
                        );
                    }
                }
                request => queued.push_back(request),
            }

            while pending.is_none()
                && let Some(request) = queued.pop_front()
            {
                pending = self.transmit(request).await;
            }
        }
    }

    async fn transmit(&mut self, message: Message) -> Option<Pending> {
        match message {
            Message::Unicast { command, response } => {
                match self.transmitter.transmit(*command).await {
                    Ok(_id) => Some(Pending::Unicast(response)),
                    Err(error) => {
                        response.send(Err(error)).ok();
                        None
                    }
                }
            }
            Message::Multicast { command, response } => {
                match self.transmitter.transmit(*command).await {
                    Ok(_id) => Some(Pending::Multicast(response)),
                    Err(error) => {
                        response.send(Err(error)).ok();
                        None
                    }
                }
            }
            Message::Broadcast { command, response } => {
                match self.transmitter.transmit(*command).await {
                    Ok(_id) => Some(Pending::Broadcast(response)),
                    Err(error) => {
                        response.send(Err(error)).ok();
                        None
                    }
                }
            }
            Message::Received(_) => None,
        }
    }
}

#[derive(Debug)]
enum Pending {
    Unicast(tokio::sync::oneshot::Sender<Result<u8, Error>>),
    Multicast(tokio::sync::oneshot::Sender<Result<u8, Error>>),
    Broadcast(tokio::sync::oneshot::Sender<Result<u8, Error>>),
}

impl Pending {
    fn complete(self, parameters: Parameters) {
        match self {
            Self::Unicast(response) => {
                response.send(unicast_response(parameters)).ok();
            }
            Self::Multicast(response) => {
                response.send(multicast_response(parameters)).ok();
            }
            Self::Broadcast(response) => {
                response.send(broadcast_response(parameters)).ok();
            }
        }
    }
}

fn unicast_response(parameters: Parameters) -> Result<u8, Error> {
    let response = send_unicast::Response::try_from(parameters).map_err(Error::from)?;
    response.try_into()
}

fn multicast_response(parameters: Parameters) -> Result<u8, Error> {
    let response = send_multicast::Response::try_from(parameters).map_err(Error::from)?;
    response.try_into()
}

fn broadcast_response(parameters: Parameters) -> Result<u8, Error> {
    let response = send_broadcast::Response::try_from(parameters).map_err(Error::from)?;
    response.try_into()
}
