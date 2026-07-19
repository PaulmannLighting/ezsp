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
//!
//! The `ASHv2` types used by the public constructors are re-exported from this
//! module. Users can import [`FlowControl`], [`Handle`], [`NativeSerialPort`],
//! [`Payload`], [`SerialPort`], [`open`], and [`start`] from
//! `ezsp::uart` without naming the underlying transport crate directly.

use core::fmt::Debug;
use core::num::TryFromIntError;
use std::borrow::Cow;
use std::io;
use std::num::NonZero;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

pub use ashv2::{FlowControl, Handle, NativeSerialPort, Payload, SerialPort, open, start};
use le_stream::ToLeStream;
use log::{debug, info, warn};
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub use self::buffers::Buffers;
pub use self::channel_sizes::ChannelSizes;
use self::encoder::Encoder;
pub use self::futures::Futures;
use crate::constants::MIN_NON_LEGACY_VERSION;
use crate::error::Error;
use crate::frame::{Command, Header, Parameter};
use crate::parameters::configuration::version;
use crate::transport::Transport;
use crate::uart::decoder::Decoder;
use crate::uart::splitter::Splitter;
use crate::{
    Builder, Callback, Configuration, Connection, Extended, Legacy, Ncp, Parameters, ValueError,
};

mod buffers;
mod channel_sizes;
mod decoder;
mod encoder;
mod futures;
mod splitter;

type SplitterFuture = Pin<Box<dyn Future<Output = io::Result<()>> + Send + 'static>>;

/// An EZSP host using `ASHv2` as the UART link layer.
#[derive(Debug)]
pub struct Uart {
    protocol_version: u8,
    negotiated_version: Arc<AtomicU8>,
    connection: Connection,
    responses: Receiver<Result<Parameters, Error>>,
    encoder: Encoder,
    sequence: u8,
}

impl Uart {
    /// Creates an EZSP host from an already running `ASHv2` link handle.
    ///
    /// This constructor is intended for integrations that start `ASHv2`
    /// themselves through [`start`] or another compatible setup and pass the
    /// resulting [`Handle`] plus inbound [`Payload`] stream into the EZSP layer.
    /// The returned future drives the EZSP frame splitter and must be polled for
    /// responses and callbacks to be routed. It resolves with an error if a
    /// destination channel closes before the splitter finishes.
    ///
    /// A minimum protocol version of [`MIN_NON_LEGACY_VERSION`] is required
    /// to support non-legacy commands.
    #[must_use]
    pub fn new(
        ash_v2: Handle,
        ash_rx: Receiver<Payload>,
        callbacks: Sender<Callback>,
        protocol_version: u8,
        channel_size: usize,
    ) -> (Self, SplitterFuture) {
        let negotiated_version = Arc::new(AtomicU8::new(0));
        let (responses_tx, responses_rx) = channel(channel_size);
        let splitter = Splitter::new(
            Decoder::new(ash_rx, negotiated_version.clone()),
            responses_tx,
            callbacks,
        )
        .run();

        let instance = Self {
            protocol_version,
            negotiated_version,
            connection: Connection::Disconnected,
            encoder: Encoder::new(ash_v2),
            responses: responses_rx,
            sequence: 0,
        };

        (instance, Box::pin(splitter))
    }

    /// Open a new EZSP-UART transport from a serial port.
    ///
    /// The serial port must implement the re-exported [`SerialPort`] trait.
    /// The returned [`Futures`] value contains the `ASHv2` actor futures and EZSP
    /// frame splitter future; all of them must be polled or spawned by the
    /// caller.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O operations fail.
    pub fn from_serial_port<T>(
        serial_port: T,
        protocol_version: u8,
        channel_sizes: &ChannelSizes,
    ) -> Result<(Self, Receiver<Callback>, Futures<T>), Error>
    where
        T: SerialPort + Sync + 'static,
    {
        let (tx, rx) = channel(channel_sizes.payload);
        let (ash_v2, futures) = start(serial_port, tx);
        let (callbacks_tx, callbacks_rx) = channel(channel_sizes.callbacks);
        let (instance, splitter) = Self::new(
            ash_v2,
            rx,
            callbacks_tx,
            protocol_version,
            channel_sizes.responses,
        );
        let futures = Futures::new(splitter, futures);
        Ok((instance, callbacks_rx, futures))
    }

    /// Open a new EZSP-UART transport from a serial port path.
    ///
    /// The path is opened through the re-exported [`open`] helper using the
    /// requested [`FlowControl`] mode. The returned [`Futures`] value must be
    /// driven by the caller for the transport to make progress.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any I/O operations fail.
    pub fn open<'a, T>(
        path: T,
        flow_control: FlowControl,
        protocol_version: u8,
        channel_sizes: &ChannelSizes,
    ) -> Result<(Self, Receiver<Callback>, Futures<impl SerialPort>), Error>
    where
        T: Into<Cow<'a, str>>,
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
            let Some(parameters) = self.responses.recv().await else {
                return Err(Error::ChannelClosed);
            };

            match T::try_from(parameters?) {
                Ok(frame) => return Ok(frame),
                Err(error) => {
                    let parameters: Parameters = error.into();
                    warn!("Dropping unexpected response: {parameters:?}");
                }
            }
        }
    }
}

#[cfg(feature = "ashv2")]
impl Ncp<Uart> {
    /// Creates a new [`Builder`] backed by an `ASHv2` UART transport.
    ///
    /// The serial port must implement [`SerialPort`], which is
    /// re-exported by the UART module from the `ASHv2` transport crate. The
    /// returned [`Futures`] value must be spawned or otherwise
    /// polled by the caller for the UART transport to make progress.
    pub fn ashv2<T>(serial_port: T) -> (Builder<Uart>, Futures<T>)
    where
        T: SerialPort + Sync + 'static,
    {
        Builder::ashv2(serial_port)
    }
}
