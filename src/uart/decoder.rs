//! Decoding of `ASHv2` frames into `EZSP` frames.

use std::io;
use std::sync::Arc;

use ashv2::{HexSlice, Payload};
use le_stream::FromLeStream;
use log::trace;
use tokio::sync::mpsc::Receiver;

use crate::error::Decode;
use crate::frame::parsable::Parsable;
use crate::frame::{Disambiguation, Frame, Header};
use crate::parameters::utilities::invalid_command;
use crate::uart::connection::Connection;
use crate::uart::np_rw_lock::NpRwLock;
use crate::uart::state::State;
use crate::{Error, Extended, Legacy, MAX_PARAMETER_SIZE, Parameters};

/// Decode `ASHv2` frames into `EZSP` frames.
#[derive(Debug)]
pub struct Decoder {
    source: Receiver<io::Result<Payload>>,
    state: Arc<NpRwLock<State>>,
    header: Option<Header>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Decoder {
    /// Create a new `Decoder`.
    ///
    /// Sets the source as a receiver for incoming `ASHv2` frames
    /// and the current state of the `EZSP` UART.
    #[must_use]
    pub const fn new(source: Receiver<io::Result<Payload>>, state: Arc<NpRwLock<State>>) -> Self {
        Self {
            source,
            state,
            header: None,
            parameters: heapless::Vec::new(),
        }
    }

    /// Decode incoming `ASHv2` frames into `EZSP` frames.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if no frame could be decoded.
    pub async fn decode(&mut self) -> Result<Frame, Error> {
        self.parameters.clear();

        loop {
            match self.source.recv().await.expect("Source closed") {
                Ok(frame) => match self.try_parse_frame_fragment(frame) {
                    Ok(maybe_frame) => {
                        if let Some(frame) = maybe_frame {
                            return Ok(frame);
                        }
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                Err(error) => {
                    self.state.write().set_connection(Connection::Failed);
                    return Err(error.into());
                }
            }
        }
    }

    /// Try to parse a frame fragment from a chunk of bytes.
    ///
    /// This function will try to parse a frame fragment from a chunk of bytes.
    ///
    /// It will parse the header and append the remaining bytes to the parameter buffer.
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
        trace!("Decoding ASHv2 frame: {:#04X}", HexSlice::new(&frame));

        let mut stream = frame.into_iter();

        let next_header = if self.state.read().is_legacy() {
            Header::Legacy(Legacy::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?)
        } else {
            Header::Extended(Extended::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?)
        };

        if let Some(header) = self.header.take()
            && header != next_header
        {
            return Err(Decode::FrameIdMismatch {
                expected: header.id(),
                found: next_header.id(),
            }
            .into());
        }

        self.parameters.extend(stream);
        let disambiguation = self.state.read().disambiguation();

        match Parameters::parse_from_le_stream(
            next_header.id(),
            disambiguation.unwrap_or_default(),
            self.parameters.iter().copied(),
        ) {
            Ok(parameters) => Ok(Some(Frame::new(next_header, parameters))),
            Err(error) => {
                if let Ok(invalid_command) = invalid_command::Response::parse_from_le_stream(
                    next_header.id(),
                    Disambiguation::None,
                    self.parameters.iter().copied(),
                ) {
                    return Err(Error::InvalidCommand(invalid_command));
                }

                if error == Decode::TooFewBytes {
                    self.header.replace(next_header);
                    return Ok(None);
                }

                Err(error.into())
            }
        }
    }
}
