//! Transport-independent actor wiring for EZSP frame I/O.
//!
//! External transports implement [`Transmit`] and [`Receive`]. [`Client::run`]
//! consumes those halves, creates the bounded actor channels, and returns a
//! pre-negotiation [`Client`] plus the [`Futures`] that drive both actors. Spawn
//! both futures before calling [`Client::connect`].

use std::num::NonZero;

use tokio::sync::mpsc::{self, Sender, channel};
use tokio::sync::oneshot;

pub use self::connection::Connection;
pub use self::futures::Futures;
use self::message::Message;
pub use self::receiver::Receive;
use self::receiver::Receiver;
pub use self::translatable_event::TranslatableEvent;
pub use self::transmitter::Transmit;
use self::transmitter::Transmitter;
use crate::{Callback, Error};

mod connection;
mod futures;
mod message;
mod receiver;
mod translatable_event;
mod transmitter;

/// Handle to running transport tasks before EZSP version negotiation.
///
/// [`Client::run`] wires transport-specific transmit and receive halves into
/// the EZSP actors. After both returned actor futures have been spawned,
/// [`Client::connect`] issues the initial `version` command and transitions to
/// a cloneable [`Connection`].
///
/// The current API does not expose a public constructor for the initial
/// `Client`; its fields are crate-private. External transport integrations
/// therefore require another API to supply the value consumed by
/// [`Client::run`].
#[derive(Debug)]
pub struct Client {
    pub(crate) handle: Sender<Message>,
    pub(crate) callbacks: mpsc::Receiver<Callback>,
}

impl Client {
    /// Replaces this client's channels and creates the transport actor futures.
    ///
    /// This method consumes the existing client, creates new command/response
    /// and callback channels around `transmit` and `receive`, and returns the
    /// newly wired client. Spawn `futures.transmitter` and `futures.receiver`
    /// before calling [`Client::connect`] or [`crate::Builder::start`]. Any
    /// lower-level tasks used by the transport implementations must also be
    /// running.
    ///
    /// `channel_size` is used for both the command/response actor channel and
    /// the asynchronous callback channel.
    ///
    /// # Panics
    ///
    /// Panics if `channel_size` is zero.
    #[must_use]
    pub fn run<T, R>(
        self,
        transmit: T,
        receive: R,
        channel_size: usize,
    ) -> (
        Self,
        Futures<
            impl Future<Output = ()> + Send + 'static,
            impl Future<Output = ()> + Send + 'static,
        >,
    )
    where
        T: Transmit + Send + 'static,
        R: Receive + Send + 'static,
    {
        let (handle, msg_rx) = channel(channel_size);
        let transmitter = Transmitter::new(transmit, msg_rx).run();
        let (cb_tx, callbacks) = channel(channel_size);
        let receiver = Receiver::new(receive, cb_tx, handle.clone()).run();
        (
            Self { handle, callbacks },
            Futures {
                transmitter,
                receiver,
            },
        )
    }

    /// Negotiates `desired_version` and returns a connection and callback stream.
    ///
    /// This method consumes the pre-negotiation client. The returned
    /// [`Connection`] is cloneable and implements the typed command traits
    /// through [`Communicate`](crate::Communicate). The callback receiver yields
    /// asynchronous EZSP callbacks routed by the receiver actor.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the actor channel closes, transmission fails, or
    /// the NCP negotiates a different protocol version.
    pub async fn connect(
        self,
        desired_version: NonZero<u8>,
    ) -> Result<(Connection, mpsc::Receiver<Callback>), Error> {
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
