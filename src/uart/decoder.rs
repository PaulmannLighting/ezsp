//! Decoding of `ASHv2` frames into `EZSP` frames.

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use ashv2::Payload;
use le_stream::FromLeStream;
use log::trace;
use tokio::sync::mpsc::Receiver;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::{Frame, Header};
use crate::parameters::utilities::invalid_command;
use crate::{
    Error, Extended, Legacy, LowByte, MAX_PARAMETER_SIZE, MIN_NON_LEGACY_VERSION, Parameters, ezsp,
};

/// Decode `ASHv2` frames into `EZSP` frames.
#[derive(Debug)]
pub struct Decoder {
    source: Receiver<Payload>,
    negotiated_version: Arc<AtomicU8>,
    header: Option<Header>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
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
            header: None,
            parameters: heapless::Vec::new(),
        }
    }

    /// Decode incoming `ASHv2` frames into `EZSP` frames.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no frame could be decoded.
    pub async fn decode(&mut self) -> Option<Result<Frame, Error>> {
        self.parameters.clear();

        while let Some(frame) = self.source.recv().await {
            match self.try_parse_frame_fragment(frame) {
                Ok(maybe_frame) => {
                    if let Some(frame) = maybe_frame {
                        return Some(Ok(frame));
                    }
                }
                Err(error) => {
                    return Some(Err(error));
                }
            }
        }

        None
    }

    /// Try to parse a frame fragment from a chunk of bytes.
    ///
    /// EZSP packets, in practice, though undocumented, may be split across multiple frames:
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
    fn try_parse_frame_fragment(&mut self, frame: Payload) -> Result<Option<Frame>, Error> {
        trace!("Decoding ASHv2 frame: {frame:#04X?}");

        let mut stream = frame.into_iter();
        let next_header = self.read_header(&mut stream).ok_or(Decode::TooFewBytes)?;
        trace!("Next header: {next_header}");

        if let LowByte::Response(response) = next_header.low_byte() {
            if response.is_truncated() {
                return Err(ezsp::Status::Error(ezsp::Error::Truncated).into());
            }

            if response.has_overflowed() {
                return Err(ezsp::Status::Error(ezsp::Error::Overflow).into());
            }
        }

        if let Some(header) = self.header.take()
            && header != next_header
        {
            self.parameters.clear();
            return Err(Decode::FrameIdMismatch {
                expected: header.id(),
                found: next_header.id(),
            }
            .into());
        }

        self.parameters.extend(stream);
        trace!("Accumulated parameters: {:#04X?}", self.parameters);

        match Parameters::parse_from_le_stream(next_header.id(), self.parameters.iter().copied()) {
            Ok(parameters) => {
                trace!("Decoded parameters: {parameters:?}");
                Ok(Some(Frame::new(next_header, parameters)))
            }
            Err(error) => self.handle_error(error, next_header),
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

    /// Handle an error that occurred during frame parsing.
    fn handle_error(&mut self, error: Decode, header: Header) -> Result<Option<Frame>, Error> {
        if let Ok(invalid_command) =
            invalid_command::Response::parse_from_le_stream(header.id(), self.parameters.drain(..))
        {
            trace!("Received invalid command error.");
            return Err(Error::InvalidCommand(invalid_command));
        }

        if error != Decode::TooFewBytes {
            trace!("Received and error during frame parsing: {error:?}");
            return Err(error.into());
        }

        trace!("Frame appears fragmented. Waiting for more data...");
        self.header.replace(header);
        Ok(None)
    }
}
