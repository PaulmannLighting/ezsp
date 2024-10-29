//! `ASHv2` transport layer.

use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::TryFromIntError;
use std::sync::{Arc, RwLock};

use le_stream::ToLeStream;
use log::debug;
use serialport::SerialPort;
use tokio::sync::mpsc::Receiver;

use crate::error::Error;
use crate::frame::{Command, Header, Identified};
use crate::parameters::configuration::version;
use crate::transport::{Transport, MIN_NON_LEGACY_VERSION};
use crate::{Configuration, Extended, Handler, Legacy};
use crate::{Parameters, ValueError};

use encoder::Encoder;
use threads::Threads;

mod decoder;
mod encoder;
mod splitter;
mod threads;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart {
    responses: Receiver<Parameters>,
    encoder: Encoder,
    _threads: Threads,
    negotiated_version: Arc<RwLock<Option<u8>>>,
    sequence: u8,
}

impl Uart {
    /// Creates an `ASHv2` host.
    #[must_use]
    pub fn new<S, H>(serial_port: S, handler: H, channel_size: usize) -> Self
    where
        S: SerialPort + 'static,
        H: Handler + 'static,
    {
        let negotiated_version = Arc::new(RwLock::new(None));
        let (frames_out, responses, threads) = Threads::spawn(
            serial_port,
            handler,
            negotiated_version.clone(),
            channel_size,
        );
        Self {
            responses,
            encoder: Encoder::new(frames_out),
            _threads: threads,
            negotiated_version,
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
    pub async fn negotiate_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> Result<version::Response, Error> {
        debug!("Negotiating legacy version");
        let mut response = self.version(desired_protocol_version).await?;

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            self.negotiated_version
                .write()
                .expect("RW lock poisoned")
                .replace(response.protocol_version());
            debug!("Negotiating non-legacy version");
            response = self.version(response.protocol_version()).await?;
        }

        if response.protocol_version() == desired_protocol_version {
            Ok(response)
        } else {
            Err(Error::ProtocolVersionMismatch {
                desired: desired_protocol_version,
                negotiated: response,
            })
        }
    }

    fn is_legacy(&self) -> bool {
        self.negotiated_version
            .read()
            .expect("Failed to read lock")
            .map(|version| version < MIN_NON_LEGACY_VERSION)
            .unwrap_or(true)
    }
}

impl Transport for Uart {
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self.is_legacy() {
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
