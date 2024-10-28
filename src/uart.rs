//! `ASHv2` transport layer.

use std::fmt::Debug;
use std::io::ErrorKind;
use std::num::TryFromIntError;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use ashv2::Payload;
use le_stream::ToLeStream;
use log::debug;
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::error::Error;
use crate::frame::{Command, Header, Identified};
use crate::parameters::configuration::version;
use crate::transport::{Transport, MIN_NON_LEGACY_VERSION};
use crate::{Configuration, Extended, Handler, Legacy, Response};
use crate::{Parameters, ValueError};

use decoder::Decoder;
use encoder::Encoder;
use splitter::Splitter;

mod decoder;
mod encoder;
mod splitter;

/// An `EZSP` host using `ASHv2` on the transport layer.
#[derive(Debug)]
pub struct Uart {
    splitter: JoinHandle<()>,
    callbacks: Receiver<Handler>,
    responses: Receiver<Parameters>,
    encoder: Encoder,
    legacy: Arc<AtomicBool>,
    sequence: u8,
}

impl Uart {
    /// Creates an `ASHv2` host.
    #[must_use]
    pub fn new(
        frames_in: Receiver<Payload>,
        frames_out: Sender<Payload>,
        channel_size: usize,
    ) -> Self {
        let legacy = Arc::new(AtomicBool::new(true));
        let (callbacks_tx, callbacks_rx) = channel(channel_size);
        let (response_tx, response_rx) = channel(channel_size);
        let decoder = Decoder::new(frames_in, legacy.clone());
        let splitter = Splitter::new(decoder, response_tx, callbacks_tx);
        Self {
            splitter: spawn(splitter.run()),
            callbacks: callbacks_rx,
            responses: response_rx,
            encoder: Encoder::new(frames_out, legacy.clone()),
            legacy,
            sequence: 0,
        }
    }

    pub async fn negotiate_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> Result<version::Response, Error> {
        debug!("Negotiating legacy version");
        let mut response = self.version(desired_protocol_version).await?;

        if response.protocol_version() >= MIN_NON_LEGACY_VERSION {
            self.legacy.store(false, Relaxed);
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
}

impl Transport for Uart {
    fn next_header(&mut self, id: u16) -> Result<Header, TryFromIntError> {
        let header = if self.legacy.load(Relaxed) {
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
