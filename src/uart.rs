//! `ASHv2` transport layer.

use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::TryFromIntError;

use le_stream::ToLeStream;
use log::{debug, warn};
use serialport::SerialPort;
use tokio::sync::mpsc::Receiver;

use crate::error::Error;
use crate::frame::{Command, Header, Identified};
use crate::parameters::configuration::version;
use crate::transport::{Transport, MIN_NON_LEGACY_VERSION};
use crate::{Configuration, Extended, Ezsp, Handler, Legacy};
use crate::{Parameters, ValueError};

use crate::uart::state::State;
use encoder::Encoder;
use threads::Threads;

mod decoder;
mod encoder;
mod splitter;
mod state;
mod threads;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart {
    protocol_version: u8,
    state: State,
    responses: Receiver<Parameters>,
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
        H: Handler + 'static,
    {
        let state = State::default();
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
    async fn negotiate_version(&mut self) -> Result<version::Response, Error> {
        debug!("Negotiating legacy version");
        let mut response = self.version(self.protocol_version).await?;

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            debug!("Negotiating non-legacy version");
            response = self.version(response.protocol_version()).await?;
        }

        if response.protocol_version() == self.protocol_version {
            Ok(response)
        } else {
            self.state.set_needs_reset(true);
            Err(Error::ProtocolVersionMismatch {
                desired: self.protocol_version,
                negotiated: response,
            })
        }
    }
}

impl Ezsp for Uart {
    async fn init(&mut self) -> Result<(), Error> {
        let version = self.negotiate_version().await?;
        self.state
            .set_negotiated_version(version.protocol_version());
        self.state.set_needs_reset(false);
        Ok(())
    }
}

impl Transport for Uart {
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self.state.is_legacy() {
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
        if self.state.needs_reset() {
            warn!("UART needs reset, reinitializing");
            self.init().await?;
        }

        Ok(())
    }

    async fn send<T>(&mut self, command: T) -> Result<(), Error>
    where
        T: Identified + ToLeStream,
    {
        let header = self
            .next_header(T::ID.into())
            .map_err(ValueError::InvalidFrameId)?;
        self.encoder.send(header, command).await
    }

    async fn receive(&mut self) -> Result<Parameters, Error> {
        let Some(response) = self.responses.recv().await else {
            return Err(
                std::io::Error::new(ErrorKind::UnexpectedEof, "Empty response from NCP.").into(),
            );
        };

        Ok(response)
    }
}
