use ashv2::{HexSlice, Payload};
use le_stream::FromLeStream;
use log::trace;
use tokio::sync::mpsc::Receiver;

use crate::error::Decode;
use crate::frame::{parsable::Parsable, Frame, Header};
use crate::parameters::utilities::invalid_command;
use crate::uart::connection::Connection;
use crate::uart::state::State;
use crate::MAX_PARAMETER_SIZE;
use crate::{Error, Extended, Legacy, Parameters};

/// Decode `ASHv2` frames into `EZSP` frames.
#[derive(Debug)]
pub struct Decoder {
    source: Receiver<std::io::Result<Payload>>,
    pub state: State,
    header: Option<Header>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Decoder {
    /// Create a new `Decoder`.
    ///
    /// Sets the source as a receiver for incoming `ASHv2` frames
    /// and the current state of the `EZSP` UART.
    #[must_use]
    pub const fn new(source: Receiver<std::io::Result<Payload>>, state: State) -> Self {
        Self {
            source,
            state,
            header: None,
            parameters: heapless::Vec::new(),
        }
    }

    /// Decode incoming `ASHv2` frames into `EZSP` frames.
    pub async fn decode(&mut self) -> Option<Result<Frame, Error>> {
        self.parameters.clear();

        loop {
            if let Some(result) = self.source.recv().await {
                match result {
                    Ok(frame) => match self.try_parse_frame_fragment(frame) {
                        Ok(Some(frame)) => {
                            return Some(Ok(frame));
                        }
                        Ok(None) => continue,
                        Err(error) => {
                            return Some(Err(error));
                        }
                    },
                    Err(error) => {
                        self.state.set_connection(Connection::Failed);
                        return Some(Err(error.into()));
                    }
                }
            }

            return None;
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
    /// Returns `Some(`[`Frame`]`)` if the frame fragment was successfully parsed.
    ///
    /// Returns `None` if the decoder needs more data to decode the frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the frame fragment could not be parsed.
    fn try_parse_frame_fragment(&mut self, frame: Payload) -> Result<Option<Frame>, Error> {
        trace!("Decoding ASHv2 frame: {:#04X}", HexSlice::new(&frame));

        let mut stream = frame.into_iter();

        let next_header = if self.state.is_legacy() {
            Header::Legacy(Legacy::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?)
        } else {
            Header::Extended(Extended::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?)
        };

        if let Some(header) = self.header.take() {
            if header != next_header {
                return Err(Decode::FrameIdMismatch {
                    expected: header.id(),
                    found: next_header.id(),
                }
                .into());
            }
        }

        self.parameters.extend(stream);

        match Parameters::parse_from_le_stream(
            next_header.id(),
            self.state.disambiguation(),
            self.parameters.iter().copied(),
        ) {
            Ok(parameters) => Ok(Some(Frame::new(next_header, parameters))),
            Err(error) => {
                if let Ok(invalid_command) = invalid_command::Response::parse_from_le_stream(
                    next_header.id(),
                    None,
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
