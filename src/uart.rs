//! `ASHv2` transport layer.

use core::fmt::Debug;
use core::num::TryFromIntError;
use std::sync::Arc;
use std::time::Duration;

use ashv2::{SerialPort, TryCloneNative};
use le_stream::ToLeStream;
use log::{debug, info, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

use self::connection::Connection;
use self::encoder::Encoder;
use self::np_rw_lock::NpRwLock;
use self::state::State;
use self::threads::Threads;
use crate::error::Error;
use crate::frame::{Command, Header, Parameter};
use crate::parameters::configuration::version;
use crate::transport::{MIN_NON_LEGACY_VERSION, Transport};
use crate::{Callback, Configuration, Extended, Ezsp, Legacy, Parameters, ValueError};

mod connection;
mod decoder;
mod encoder;
mod np_rw_lock;
mod splitter;
mod state;
mod threads;

const REQUEUE_GRACE_PERIOD: Duration = Duration::from_millis(100);

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart {
    protocol_version: u8,
    state: Arc<NpRwLock<State>>,
    responses_tx: Sender<Result<Parameters, Error>>,
    responses_rx: Receiver<Result<Parameters, Error>>,
    encoder: Encoder,
    threads: Threads,
    sequence: u8,
}

impl Uart {
    /// Creates an `ASHv2` host.
    ///
    /// A minimum protocol version of [`MIN_NON_LEGACY_VERSION`] is required
    /// to support non-legacy commands.
    #[must_use]
    pub fn new<T>(
        serial_port: T,
        callbacks: Sender<Callback>,
        protocol_version: u8,
        channel_size: usize,
    ) -> Self
    where
        T: SerialPort + TryCloneNative + Sync + 'static,
    {
        let state = Arc::new(NpRwLock::new(State::default()));
        let (frames_out, responses_tx, responses_rx, threads) =
            Threads::spawn(serial_port, callbacks, state.clone(), channel_size);
        Self {
            protocol_version,
            state,
            responses_tx,
            responses_rx,
            encoder: Encoder::new(frames_out),
            threads,
            sequence: 0,
        }
    }

    /// Return the next header.
    ///
    /// This method is used to determine the next header to be used in the communication.
    ///
    /// The `id` parameter is the identifier of the command that will be sent.
    ///
    /// # Errors
    ///
    /// This method may return an error if `EZSP` is in legacy mode
    /// and the `id` cannot be converted into a `u8`.
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self.state.read().is_legacy() {
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

    /// Negotiate the `EZSP` protocol version.
    ///
    /// A minimum version of [`MIN_NON_LEGACY_VERSION`] is required to support non-legacy commands.
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
        self.state
            .write()
            .set_negotiated_version(response.protocol_version());

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            debug!("Negotiating non-legacy version");
            response = self.version(response.protocol_version()).await?;
            self.state
                .write()
                .set_negotiated_version(response.protocol_version());
        }

        if response.protocol_version() == self.protocol_version {
            info!(
                "Negotiated protocol version: {:#04X}",
                response.protocol_version()
            );
            Ok(response)
        } else {
            self.state.write().set_connection(Connection::Failed);
            Err(Error::ProtocolVersionMismatch {
                desired: self.protocol_version,
                negotiated: response,
            })
        }
    }

    /// Abort the UART threads.
    ///
    /// # Errors
    ///
    /// Returns a [`JoinError`] if any of the threads fail to abort.
    pub async fn abort(self) {
        self.threads.abort().await;
    }
}

impl Ezsp for Uart {
    async fn init(&mut self) -> Result<version::Response, Error> {
        let response = self.negotiate_version().await?;
        self.state.write().set_connection(Connection::Connected);
        Ok(response)
    }

    fn negotiated_version(&self) -> Option<u8> {
        self.state.read().negotiated_version()
    }
}

impl Transport for Uart {
    async fn check_reset(&mut self) -> Result<(), Error> {
        // Use temporary variable, because we need to drop the lock before the match statement.
        let connection = self.state.read().connection();

        match connection {
            Connection::Disconnected => {
                info!("Initializing UART connection");
                self.init().await.map(drop)
            }
            Connection::Connected => {
                trace!("UART is connected");
                Ok(())
            }
            Connection::Failed => {
                warn!("UART connection failed, reinitializing");
                self.init().await.map(drop)
            }
        }
    }

    async fn send<C>(&mut self, command: C) -> Result<u16, Error>
    where
        C: Parameter + ToLeStream,
    {
        let header = self
            .next_header(C::ID)
            .map_err(ValueError::InvalidFrameId)?;
        let id = header.id();
        // Set disambiguation for the command being sent.
        //
        // XXX: This needs to be done before sending the command, because if the serial port
        // responds before we set the disambiguation, we might misinterpret the response.
        self.state.write().set_disambiguation(C::DISAMBIGUATION);
        self.encoder.send(header, command).await?;
        Ok(id)
    }

    async fn receive<P>(&mut self) -> Result<P, Error>
    where
        P: TryFrom<Parameters> + Send,
        <P as TryFrom<Parameters>>::Error: Into<Parameters> + Send,
    {
        let mut parameters;

        loop {
            parameters = self
                .responses_rx
                .recv()
                .await
                .expect("Response channel should be open. This is a bug.")?;

            match P::try_from(parameters) {
                Ok(frame) => return Ok(frame),
                Err(error) => {
                    parameters = error.into();
                    trace!(
                        "Received unexpected response: {parameters:?}, re-queueing and retrying in {REQUEUE_GRACE_PERIOD:?}."
                    );
                    sleep(REQUEUE_GRACE_PERIOD).await;
                    self.responses_tx
                        .send(Ok(parameters))
                        .await
                        .expect("Re-queueing channel should be open. This is a bug.");
                }
            }
        }
    }
}
