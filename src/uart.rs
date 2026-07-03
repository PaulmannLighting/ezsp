//! EZSP-UART transport built on `ASHv2`.
//!
//! Silicon Labs describes `ASHv2` as the data-link layer below EZSP for UART
//! gateway systems. `ASHv2` carries EZSP DATA payloads and supplies reset
//! handling, CRC validation, byte stuffing, data-field randomization, ACK/NAK
//! frames, and sliding-window retransmission.
//!
//! This module delegates `ASHv2` link handling to the `ashv2` crate and handles
//! the EZSP-specific pieces: protocol-version negotiation, EZSP header
//! selection, typed request/response exchange, and asynchronous callback
//! demultiplexing.

use core::fmt::Debug;
use core::num::TryFromIntError;
use std::borrow::Cow;
use std::num::NonZero;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

use ashv2::{
    Actor, FlowControl, NativeSerialPort, Payload, Proxy, SerialPort, Tasks, TryCloneNative, open,
};
use le_stream::ToLeStream;
use log::{debug, error, info, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio::task::{JoinError, JoinHandle};
use tokio::time::sleep;
use tokio_task_pool::Pool;

pub use self::buffers::Buffers;
pub use self::channel_sizes::ChannelSizes;
use self::encoder::Encoder;
use crate::constants::MIN_NON_LEGACY_VERSION;
use crate::error::Error;
use crate::frame::{Command, Header, Parameter};
use crate::parameters::configuration::version;
use crate::transport::Transport;
use crate::uart::decoder::Decoder;
use crate::uart::splitter::Splitter;
use crate::{Callback, Configuration, Connection, Extended, Legacy, Parameters, ValueError};

mod buffers;
mod channel_sizes;
mod decoder;
mod encoder;
mod splitter;

const REQUEUE_GRACE_PERIOD: Duration = Duration::from_millis(100);

/// An EZSP host using `ASHv2` as the UART link layer.
#[derive(Debug)]
pub struct Uart {
    protocol_version: u8,
    negotiated_version: Arc<AtomicU8>,
    connection: Connection,
    responses_tx: Sender<Result<Parameters, Error>>,
    responses_rx: Receiver<Result<Parameters, Error>>,
    encoder: Encoder,
    splitter: JoinHandle<()>,
    sequence: u8,
    tasks: Pool,
}

impl Uart {
    /// Creates an EZSP host from an already running `ASHv2` proxy.
    ///
    /// A minimum protocol version of [`MIN_NON_LEGACY_VERSION`] is required
    /// to support non-legacy commands.
    #[must_use]
    pub fn new(
        ash_proxy: Proxy,
        ash_rx: Receiver<Payload>,
        callbacks: Sender<Callback>,
        protocol_version: u8,
        channel_size: usize,
    ) -> Self {
        let negotiated_version = Arc::new(AtomicU8::new(0));
        let (responses_tx, responses_rx) = channel(channel_size);
        let splitter = spawn(
            Splitter::new(
                Decoder::new(ash_rx, negotiated_version.clone()),
                responses_tx.clone(),
                callbacks,
            )
            .run(),
        );

        Self {
            protocol_version,
            negotiated_version,
            connection: Connection::Disconnected,
            encoder: Encoder::new(ash_proxy),
            responses_tx,
            responses_rx,
            splitter,
            sequence: 0,
            tasks: Pool::bounded(channel_size),
        }
    }

    /// Open a new EZSP-UART transport from a serial port.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O operations fail.
    pub fn from_serial_port<P>(
        serial_port: P,
        protocol_version: u8,
        channel_sizes: &ChannelSizes,
    ) -> Result<(Self, Tasks<P>, Receiver<Callback>), Error>
    where
        P: SerialPort + TryCloneNative + Sync + 'static,
    {
        let (tx, rx) = channel(channel_sizes.payload);
        let (tasks, proxy) = Actor::new(serial_port, tx, channel_sizes.message_queue)
            .map_err(|error| Error::Io(error.into()))?
            .spawn();
        let (callbacks_tx, callbacks_rx) = channel(channel_sizes.callbacks);
        Ok((
            Self::new(
                proxy,
                rx,
                callbacks_tx,
                protocol_version,
                channel_sizes.responses,
            ),
            tasks,
            callbacks_rx,
        ))
    }

    /// Open a new EZSP-UART transport from a serial port path.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O operations fail.
    pub fn open<'a, P>(
        path: P,
        flow_control: FlowControl,
        protocol_version: u8,
        channel_sizes: &ChannelSizes,
    ) -> Result<(Self, Tasks<NativeSerialPort>, Receiver<Callback>), Error>
    where
        P: Into<Cow<'a, str>>,
    {
        Self::from_serial_port(
            open(path, flow_control).map_err(|error| Error::Io(error.into()))?,
            protocol_version,
            channel_sizes,
        )
    }

    /// Return the next EZSP header.
    ///
    /// Protocol versions before 8 use the legacy three-byte EZSP header. Version
    /// 8 and newer use the extended five-byte header with a two-byte frame
    /// control field and a two-byte frame ID.
    ///
    /// The `id` parameter is the identifier of the command that will be sent.
    ///
    /// # Errors
    ///
    /// This method may return an error if `EZSP` is in legacy mode
    /// and the `id` cannot be converted into a `u8`.
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self.negotiated_version.load(Ordering::Relaxed) < MIN_NON_LEGACY_VERSION {
            Header::Legacy(Legacy::new(
                self.sequence,
                Command::default().into(),
                id.try_into()?,
            ))
        } else {
            Header::Extended(Extended::new(self.sequence, Command::default().into(), id))
        };
        self.sequence = self.sequence.wrapping_add(1);
        Ok(header)
    }

    /// Negotiate the EZSP protocol version.
    ///
    /// The Silicon Labs reference specifies that the host starts with the
    /// `version` command after NCP reset. This implementation first sends that
    /// command with the legacy frame format, then repeats it with the negotiated
    /// non-legacy format when the NCP reports protocol version 8 or newer.
    ///
    /// # Errors
    ///
    /// Returns an error on I/O errors or if the desired protocol version is not supported.
    ///
    /// # Panics
    ///
    /// Panics if the read-write lock is poisoned.
    async fn negotiate_version(&mut self) -> Result<version::Response, Error> {
        debug!("Negotiating legacy version");
        let mut response = self.version(self.protocol_version).await?;
        self.negotiated_version
            .store(response.protocol_version(), Ordering::Relaxed);

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            debug!("Negotiating non-legacy version");
            response = self.version(response.protocol_version()).await?;
            self.negotiated_version
                .store(response.protocol_version(), Ordering::Relaxed);
        }

        if response.protocol_version() == self.protocol_version {
            info!(
                "Negotiated protocol version: {:#04X}",
                response.protocol_version()
            );
            Ok(response)
        } else {
            self.connection = Connection::Failed;
            Err(Error::ProtocolVersionMismatch {
                desired: self.protocol_version,
                negotiated: response,
            })
        }
    }

    /// Abort the UART background task.
    ///
    /// # Errors
    ///
    /// Returns a [`JoinError`] if any of the threads fail to abort.
    pub async fn abort(self) -> Result<(), JoinError> {
        self.splitter.abort();
        self.splitter.await
    }
}

impl Transport for Uart {
    async fn connect(&mut self) -> Result<version::Response, Error> {
        let response = self.negotiate_version().await?;
        self.connection = Connection::Connected;
        Ok(response)
    }

    fn state(&self) -> Connection {
        self.connection
    }

    fn negotiated_version(&self) -> Option<NonZero<u8>> {
        NonZero::<u8>::new(self.negotiated_version.load(Ordering::Relaxed))
    }

    async fn send<T>(&mut self, command: T) -> Result<u16, Error>
    where
        T: Parameter + ToLeStream,
    {
        let header = self
            .next_header(T::ID)
            .map_err(ValueError::InvalidFrameId)?;
        let id = header.id();
        self.encoder.send(header, command).await?;
        Ok(id)
    }

    async fn receive<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<Parameters> + Send,
        <T as TryFrom<Parameters>>::Error: Into<Parameters> + Send,
    {
        loop {
            let Some(parameters) = self.responses_rx.recv().await else {
                return Err(Error::ChannelClosed);
            };

            match T::try_from(parameters?) {
                Ok(frame) => return Ok(frame),
                Err(error) => {
                    self.tasks
                        .spawn(requeue(
                            self.responses_tx.downgrade(),
                            error.into(),
                            REQUEUE_GRACE_PERIOD,
                        ))
                        .await
                        .map_or_else(
                            |error| {
                                error!("Failed to re-queue response: {error:?}");
                            },
                            drop,
                        );
                }
            }
        }
    }
}

/// Async task to requeue in the background.
async fn requeue(
    responses: WeakSender<Result<Parameters, Error>>,
    parameters: Parameters,
    delay: Duration,
) {
    warn!(
        "Received unexpected response: {parameters:?}, re-queueing in {}ms.",
        delay.as_millis()
    );
    sleep(delay).await;

    let Some(responses) = responses.upgrade() else {
        trace!("Re-queueing channel is closed, aborting");
        return;
    };

    responses
        .send(Ok(parameters))
        .await
        .expect("Re-queueing channel should be open. This is a bug.");
}
