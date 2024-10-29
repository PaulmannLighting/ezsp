use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use ashv2::{HexSlice, Payload};
use le_stream::FromLeStream;
use log::trace;
use tokio::sync::mpsc::Receiver;

use crate::error::Decode;
use crate::frame::{parsable::Parsable, Frame, Header};
use crate::parameters::utilities::invalid_command;
use crate::MAX_PARAMETER_SIZE;
use crate::{Error, Extended, Legacy, Parameters};

/// Decode `ASHv2` frames into `EZSP` frames.
#[derive(Debug)]
pub struct Decoder {
    source: Receiver<Payload>,
    legacy: Arc<AtomicBool>,
    header: Option<Header>,
    parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Decoder {
    #[must_use]
    pub const fn new(source: Receiver<Payload>, legacy: Arc<AtomicBool>) -> Self {
        Self {
            source,
            legacy,
            header: None,
            parameters: heapless::Vec::new(),
        }
    }

    pub async fn decode(&mut self) -> Option<Result<Frame, Error>> {
        self.parameters.clear();

        loop {
            if let Some(frame) = self.source.recv().await {
                match self.try_parse_frame_fragment(frame) {
                    Ok(Some(frame)) => {
                        return Some(Ok(frame));
                    }
                    Ok(None) => continue,
                    Err(error) => {
                        return Some(Err(error));
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
    /// Returns `Ok(Some(header))` if the frame fragment was successfully parsed.
    ///
    /// Returns `Ok(None)` if the frame is not yet complete.
    ///
    /// # Errors
    ///
    /// Returns `Err(Decode)` if the frame fragment could not be parsed.
    fn try_parse_frame_fragment(&mut self, frame: Payload) -> Result<Option<Frame>, Error> {
        trace!("Decoding ASHv2 frame: {:#04X}", HexSlice::new(&frame));

        let mut stream = frame.into_iter();
        let next_header = if self.legacy.load(Relaxed) {
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

        match Parameters::parse_from_le_stream(next_header.id(), self.parameters.iter().copied()) {
            Ok(parameters) => Ok(Some(Frame::new(next_header, parameters))),
            Err(error) => {
                if let Ok(invalid_command) = invalid_command::Response::parse_from_le_stream(
                    next_header.id(),
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
