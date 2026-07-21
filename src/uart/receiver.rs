use ashv2::Payload;
use le_stream::FromLeStream;
use log::{trace, warn};
use tokio::sync::mpsc;

use crate::error::Decode;
use crate::frame::parsable::WarnExcessBytes;
use crate::parameters::utilities::invalid_command;
use crate::{
    Error, Extended, Frame, Header, Legacy, LowByte, MIN_NON_LEGACY_VERSION, Parameters, Parsable,
    Receive, ezsp,
};

/// Receives `ASHv2` DATA payloads and decodes them as typed EZSP frames.
pub struct Receiver {
    inbox: mpsc::Receiver<Payload>,
    negotiated_version: Option<u8>,
}

impl Receiver {
    /// Creates a receiver over the `ASHv2` DATA payload channel.
    #[must_use]
    pub const fn new(inbox: mpsc::Receiver<Payload>) -> Self {
        Self {
            inbox,
            negotiated_version: None,
        }
    }

    fn read_header<T>(&self, stream: T) -> Option<Header>
    where
        T: Iterator<Item = u8>,
    {
        if self
            .negotiated_version
            .is_some_and(|version| version >= MIN_NON_LEGACY_VERSION)
        {
            Extended::from_le_stream(stream).map(Header::Extended)
        } else {
            Legacy::from_le_stream(stream).map(Header::Legacy)
        }
    }

    /// Parse one EZSP frame from one `ASHv2` DATA payload.
    ///
    /// EZSP has no protocol-level fragmentation, and `ASHv2` DATA payloads carry
    /// complete EZSP frames. The first bytes are parsed as the EZSP header; all
    /// remaining bytes are parsed as the frame parameters.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the frame could not be parsed.
    fn parse_frame(&self, frame: Payload) -> Result<Frame<Parameters>, Error> {
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

        trace!("Accumulated parameters: {stream:#04X?}");

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
}

impl Receive for Receiver {
    async fn receive(&mut self) -> Option<Frame<Parameters>> {
        loop {
            let frame = self.inbox.recv().await?;

            match self.parse_frame(frame) {
                Ok(frame) => return Some(frame),
                Err(error) => {
                    warn!("{error}");
                }
            }
        }
    }

    fn set_negotiated_version(&mut self, version: u8) -> Option<u8> {
        self.negotiated_version.replace(version)
    }
}
