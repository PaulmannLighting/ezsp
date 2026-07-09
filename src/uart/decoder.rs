//! Decoding of `ASHv2` DATA payloads into EZSP frames.
//!
//! `ASHv2` validates, acknowledges, un-stuffs, and de-randomizes UART frames
//! before this decoder sees their DATA fields. The decoder is responsible for
//! parsing the EZSP header and accumulating the EZSP parameters into a typed
//! [`Frame`].

use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use ashv2::Payload;
use le_stream::FromLeStream;
use log::trace;
use tokio::sync::mpsc::Receiver;

use crate::error::Decode;
use crate::frame::parsable::{Parsable, WarnExcessBytes};
use crate::frame::{Frame, Header};
use crate::parameters::utilities::invalid_command;
use crate::{Error, Extended, Legacy, LowByte, MIN_NON_LEGACY_VERSION, Parameters, ezsp};

/// Decodes `ASHv2` payloads into typed EZSP frames.
#[derive(Debug)]
pub struct Decoder {
    source: Receiver<Payload>,
    negotiated_version: Arc<AtomicU8>,
}

impl Decoder {
    /// Create a new `Decoder`.
    ///
    /// Sets the source as a receiver for incoming `ASHv2` frames
    /// and the current state of the `EZSP` UART.
    #[must_use]
    pub const fn new(source: Receiver<Payload>, negotiated_version: Arc<AtomicU8>) -> Self {
        Self {
            source,
            negotiated_version,
        }
    }

    /// Decode incoming `ASHv2` payloads into one EZSP frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no frame could be decoded.
    pub async fn decode(&mut self) -> Option<Result<Frame, Error>> {
        let frame = self.source.recv().await?;
        Some(self.parse_frame(frame))
    }

    /// Try to parse a frame fragment from a chunk of bytes.
    ///
    /// `ASHv2` DATA fields are limited by the UART link layer. Large EZSP
    /// parameter bodies may therefore arrive split across multiple `ASHv2`
    /// payloads:
    ///
    /// <EZSP Header><Payload Fragment 1>, <EZSP Header><Payload Fragment 2>, ...
    ///
    /// This method will parse these potentially fragmented EZSP frames by matching the headers
    /// and appending the remaining bytes to the parameter buffer.
    ///
    /// # Returns
    ///
    /// Returns <code>Some([Frame])</code> if the frame fragment was successfully parsed.
    ///
    /// Returns `None` if the decoder needs more data to decode the frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the frame fragment could not be parsed.
    fn parse_frame(&self, frame: Payload) -> Result<Frame, Error> {
        trace!("Decoding ASHv2 frame: {frame:#04X?}");

        let mut stream = frame.into_iter();
        let header = self.read_header(&mut stream).ok_or(Decode::TooFewBytes)?;
        trace!("Decoded header: {header}");

        if let LowByte::Response(response) = header.low_byte() {
            if response.is_truncated() {
                return Err(ezsp::Status::Error(ezsp::Error::Truncated).into());
            }

            if response.has_overflowed() {
                return Err(ezsp::Status::Error(ezsp::Error::Overflow).into());
            }
        }

        trace!("Accumulated parameters: {:#04X?}", stream);

        if header.id() == invalid_command::Response::ID {
            return Err(invalid_command::Response::from_le_stream_exact(&mut stream)
                .warn_excess_bytes(stream)?
                .into());
        }

        match Parameters::parse_from_le_stream(header.id(), stream) {
            Ok(parameters) => {
                trace!("Decoded parameters: {parameters:?}");
                Ok(Frame::new(header, parameters))
            }
            Err(error) => Err(error.into()),
        }
    }

    /// Read the header from a stream of bytes.
    fn read_header<T>(&self, stream: T) -> Option<Header>
    where
        T: Iterator<Item = u8>,
    {
        if self.negotiated_version.load(Ordering::Relaxed) < MIN_NON_LEGACY_VERSION {
            Legacy::from_le_stream(stream).map(Header::Legacy)
        } else {
            Extended::from_le_stream(stream).map(Header::Extended)
        }
    }
}
