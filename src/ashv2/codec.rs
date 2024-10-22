use ashv2::{HexSlice, MAX_PAYLOAD_SIZE};
use le_stream::{FromLeStream, ToLeStream};
use log::{debug, trace, warn};
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
pub struct Codec<'a, H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    header: Option<H>,
    _parameter: std::marker::PhantomData<P>,
    buffers: Buffers,
    expected_frames: &'a mut usize,
}

impl<'a, H, P> Codec<'a, H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    pub fn new(expected_frames: &'a mut usize) -> Self {
        Self {
            header: None,
            _parameter: std::marker::PhantomData,
            buffers: Buffers::default(),
            expected_frames,
        }
    }
}

impl<'a, H, P> Codec<'a, H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
    /// Try to parse a frame `Frame<H, P>` from a stream of bytes.
    ///
    /// # Returns
    ///
    /// Returns `Ok(frame)` if the frame was successfully parsed.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error)` if the frame could not be parsed.
    fn try_parse_frame<'frames>(
        &mut self,
        frames: impl Iterator<Item = &'frames [u8]>,
    ) -> Result<Frame<H, P>, Error> {
        self.buffers.parameters.clear();
        let mut header: Option<H> = None;

        for frame in frames {
            header = self.try_parse_frame_fragment(frame)?;
        }

        let Some(header) = header else {
            return Err(Decode::TooFewBytes.into());
        };

        if header.id().into() == invalid_command::Response::ID {
            return Err(Error::InvalidCommand(
                invalid_command::Response::from_le_stream_exact(
                    self.buffers.parameters.iter().copied(),
                )?,
            ));
        }

        Ok(Frame::new(
            header,
            P::parse_from_le_stream(header.id().into(), self.buffers.parameters.iter().copied())?,
        ))
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
    fn try_parse_frame_fragment(&mut self, frame: &[u8]) -> Result<Option<H>, Decode> {
        trace!("Decoding ASHv2 frame: {:#04X}", HexSlice::new(frame));

        let mut stream = frame.iter().copied();
        let next_header = H::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?;
        let mut header: Option<H> = None;

        if let Some(header) = header {
            if header.id() != next_header.id() || header.sequence() != next_header.sequence() {
                return Err(Decode::FrameIdMismatch {
                    expected: header.id().into(),
                    found: next_header.id().into(),
                });
            }
        } else {
            header.replace(next_header);
        }

        self.buffers.parameters.extend(stream);
        Ok(header)
    }
}

impl<'a, H, P> Decoder for Codec<'a, H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
    type Item = Frame<H, P>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        trace!("Decoding ASHv2 frame from buffer of size {}", src.len());
        let expected_frames = (*self.expected_frames).clamp(1, src.len());
        *self.expected_frames = 0;
        trace!("Attempting partial decoding with {expected_frames} expected frames");

        let chunk_size = src.len() / expected_frames;
        trace!("Set chunk size to {chunk_size}");
        let mut last_error: Option<Error> = None;

        // Try to decode all partial frames from `self.expected_frames` < n <= 1.
        for n in (1..=expected_frames).rev() {
            trace!("Trying with {n} chunks");

            match self.try_parse_frame(src.chunks(chunk_size).take(n)) {
                Ok(frame) => {
                    if n != expected_frames {
                        debug!("Successfully decoded frame from partial stream using {n}/{expected_frames} chunks of size {chunk_size}");
                    }

                    src.clear();
                    self.header.take();
                    return Ok(Some(frame));
                }
                Err(error) => {
                    warn!("Failed to decode frame from partial stream using {n} chunks of size {chunk_size}");
                    trace!("Error: {error}");
                    last_error.replace(error);
                }
            }
        }

        // If we have too few bytes to decode a frame, return `None` to let the stream gather more bytes.
        last_error.map_or_else(
            || Ok(None),
            |error| match error {
                Error::Decode(Decode::TooFewBytes) => Ok(None),
                error => Err(error),
            },
        )
    }
}

impl<'a, H, P> Encoder<Frame<H, P>> for Codec<'a, H, P>
where
    H: Header<P::Id>,
    P: Parameter + ToLeStream,
{
    type Error = Error;

    fn encode(&mut self, item: Frame<H, P>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        *self.expected_frames = 0;
        self.buffers.clear();
        self.buffers.header.extend(item.header().to_le_stream());
        self.buffers
            .parameters
            .extend(item.parameters().to_le_stream());

        if self.buffers.parameters.is_empty() {
            // If there are no parameters to send, e.g. on `nop`, a call to `.chunks()`
            // would yield an empty iterator, resulting in us not even sending one chunk.
            dst.extend_from_slice(&self.buffers.header);
            *self.expected_frames = 1;
            return Ok(());
        }

        for chunk in self
            .buffers
            .parameters
            .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.buffers.header.len()))
        {
            dst.extend_from_slice(&self.buffers.header);
            dst.extend_from_slice(chunk);
            *self.expected_frames = self.expected_frames.saturating_add(1);
        }

        Ok(())
    }
}
