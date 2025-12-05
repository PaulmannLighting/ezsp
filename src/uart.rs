//! `ASHv2` transport layer.

use core::fmt::Debug;
use core::num::TryFromIntError;
use std::sync::Arc;

use ashv2::SerialPort;
use le_stream::ToLeStream;
use log::{debug, info, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender};

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

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart<T> {
    protocol_version: u8,
    state: Arc<NpRwLock<State>>,
    responses: Receiver<Result<Parameters, Error>>,
    encoder: Encoder,
    threads: Threads<T>,
    sequence: u8,
}

impl<T> Uart<T>
where
    T: SerialPort,
{
    /// Creates an `ASHv2` host.
    ///
    /// A minimum protocol version of [`MIN_NON_LEGACY_VERSION`] is required
    /// to support non-legacy commands.
    #[must_use]
    pub fn new(
        serial_port: T,
        callbacks: Sender<Callback>,
        protocol_version: u8,
        channel_size: usize,
    ) -> Self
    where
        T: 'static,
    {
        let state = Arc::new(NpRwLock::new(State::default()));
        let (frames_out, responses, threads) =
            Threads::spawn(serial_port, callbacks, state.clone(), channel_size);
        Self {
            protocol_version,
            state,
            responses,
            encoder: Encoder::new(frames_out),
            threads,
            sequence: 0,
        }
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

    /// Terminate the UART threads and return the serial port.
    #[must_use]
    pub fn terminate(self) -> T {
        self.threads.terminate()
    }
}

impl<T> Ezsp for Uart<T>
where
    T: SerialPort,
{
    async fn init(&mut self) -> Result<version::Response, Error> {
        let response = self.negotiate_version().await?;
        self.state.write().set_connection(Connection::Connected);
        Ok(response)
    }
}

impl<T> Transport for Uart<T>
where
    T: SerialPort,
{
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

    async fn send<C>(&mut self, command: C) -> Result<(), Error>
    where
        C: Parameter + ToLeStream,
    {
        let header = self
            .next_header(C::ID)
            .map_err(ValueError::InvalidFrameId)?;
        // Set disambiguation for the command being sent.
        //
        // XXX: This needs to be done before sending the command, because if the serial port
        // responds before we set the disambiguation, we might misinterpret the response.
        self.state.write().set_disambiguation(C::DISAMBIGUATION);
        self.encoder.send(header, command).await?;
        Ok(())
    }

    async fn receive<P>(&mut self) -> Result<P, Error>
    where
        P: TryFrom<Parameters>,
        Error: From<<P as TryFrom<Parameters>>::Error>,
    {
        let response = self
            .responses
            .recv()
            .await
            .expect("Response channel should be open. This is a bug.");
        Ok(response?.try_into()?)
    }
}
