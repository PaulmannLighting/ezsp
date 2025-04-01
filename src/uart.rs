//! `ASHv2` transport layer.

use std::fmt::Debug;
use std::num::TryFromIntError;
use std::sync::Arc;

use le_stream::ToLeStream;
use log::{debug, info, trace, warn};
use serialport::SerialPort;
use tokio::sync::mpsc::Receiver;

use crate::error::Error;
use crate::frame::{Command, Header, Parameter};
use crate::transport::{MIN_NON_LEGACY_VERSION, Transport};
use crate::util::NpRwLock;
use crate::{Configuration, Extended, Ezsp, Handler, Legacy};
use crate::{Parameters, ValueError};

use connection::Connection;
use encoder::Encoder;
use state::State;
use threads::Threads;

mod connection;
mod decoder;
mod encoder;
mod splitter;
mod state;
mod threads;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart {
    protocol_version: u8,
    state: Arc<NpRwLock<State>>,
    responses: Receiver<Result<Parameters, Error>>,
    encoder: Encoder,
    _threads: Threads,
    sequence: u8,
}

impl Uart {
    /// Creates an `ASHv2` host.
    ///
    /// A minimum protocol version of [`MIN_NON_LEGACY_VERSION`] is required
    /// to support non-legacy commands.
    #[must_use]
    pub fn new<S, H>(serial_port: S, handler: H, protocol_version: u8, channel_size: usize) -> Self
    where
        S: SerialPort + 'static,
        H: Handler,
    {
        let state = Arc::new(NpRwLock::new(State::default()));
        let (frames_out, responses, threads) =
            Threads::spawn(serial_port, handler, state.clone(), channel_size);
        Self {
            protocol_version,
            state,
            responses,
            encoder: Encoder::new(frames_out),
            _threads: threads,
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
    async fn negotiate_version(&mut self) -> Result<(), Error> {
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
            Ok(())
        } else {
            self.state.write().set_connection(Connection::Failed);
            Err(Error::ProtocolVersionMismatch {
                desired: self.protocol_version,
                negotiated: response,
            })
        }
    }
}

impl Ezsp for Uart {
    async fn init(&mut self) -> Result<(), Error> {
        self.negotiate_version().await?;
        self.state.write().set_connection(Connection::Connected);
        Ok(())
    }
}

impl Transport for Uart {
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
                self.init().await
            }
            Connection::Connected => {
                trace!("UART is connected");
                Ok(())
            }
            Connection::Failed => {
                warn!("UART connection failed, reinitializing");
                self.init().await
            }
        }
    }

    async fn send<T>(&mut self, command: T) -> Result<(), Error>
    where
        T: Parameter + ToLeStream,
    {
        let header = self
            .next_header(T::ID)
            .map_err(ValueError::InvalidFrameId)?;
        self.encoder.send(header, command).await?;
        self.state.write().set_disambiguation(T::DISAMBIGUATION);
        Ok(())
    }

    async fn receive<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<Parameters>,
        Error: From<<T as TryFrom<Parameters>>::Error>,
    {
        let response = self
            .responses
            .recv()
            .await
            .expect("Response channel should be open. This is a bug");
        Ok(response?.try_into()?)
    }
}
