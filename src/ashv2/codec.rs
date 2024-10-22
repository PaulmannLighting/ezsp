use ashv2::{Frames, HexSlice, MAX_PAYLOAD_SIZE};
use le_stream::ToLeStream;
use log::trace;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::error::Decode;
use crate::frame::{Frame, Header, Parameter};
use crate::Error;

use crate::ashv2::parsable::Parsable;

use crate::parameters::utilities::invalid_command;
use buffers::Buffers;

mod buffers;

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::codec::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    header: Option<H>,
    _parameter: std::marker::PhantomData<P>,
    buffers: Buffers,
}

impl<H, P> Default for Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    fn default() -> Self {
        Self {
            header: None,
            _parameter: std::marker::PhantomData,
            buffers: Buffers::default(),
        }
    }
}

impl<H, P> Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
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
    fn try_parse_frame_fragment(&mut self) -> Result<Option<Frame<H, P>>, Error> {
        trace!(
            "Decoding ASHv2 frame: {:#04X}",
            HexSlice::new(&self.buffers.frame)
        );

        let mut stream = self.buffers.frame.iter().copied();
        let next_header = H::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?;

        if let Some(header) = self.header.take() {
            if header != next_header {
                return Err(Decode::FrameIdMismatch {
                    expected: header.id().into(),
                    found: next_header.id().into(),
                }
                .into());
            }
        }

        self.buffers.parameters.extend(stream);

        match P::parse_from_le_stream(
            next_header.id().into(),
            self.buffers.parameters.iter().copied(),
        ) {
            Ok(parameters) => Ok(Some(Frame::new(next_header, parameters))),
            Err(error) => {
                if let Ok(invalid_command) = invalid_command::Response::parse_from_le_stream(
                    next_header.id().into(),
                    self.buffers.parameters.iter().copied(),
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

impl<H, P> Decoder for Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
    type Item = Frame<H, P>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.buffers.clear();
        let mut stream = src.iter().copied();

        while stream.buffer_next(&mut self.buffers.frame) == Some(()) {
            match self.try_parse_frame_fragment() {
                Ok(Some(frame)) => {
                    return Ok(Some(frame));
                }
                Ok(None) => continue,
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(None)
    }
}

impl<H, P> Encoder<Frame<H, P>> for Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter + ToLeStream,
{
    type Error = Error;

    fn encode(&mut self, item: Frame<H, P>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.buffers.clear();
        self.buffers.header.extend(item.header().to_le_stream());
        self.buffers
            .parameters
            .extend(item.parameters().to_le_stream());

        if self.buffers.parameters.is_empty() {
            // If there are no parameters to send, e.g. on `nop`, a call to `.chunks()`
            // would yield an empty iterator, resulting in us not even sending one chunk.
            dst.extend_from_slice(&self.buffers.header);
            return Ok(());
        }

        for chunk in self
            .buffers
            .parameters
            .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.buffers.header.len()))
        {
            dst.extend_from_slice(&self.buffers.header);
            dst.extend_from_slice(chunk);
        }

        Ok(())
    }
}
