use le_stream::ToLeStream;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::constants::{MAX_HEADER_SIZE, MAX_PARAMETER_SIZE};
use crate::error::Decode;
use crate::frame::{Frame, Header, Parameter};
use crate::Error;

use crate::ashv2::parsable::Parsable;
use ashv2::MAX_PAYLOAD_SIZE;
use log::debug;

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
    expected_frames: usize,
}

impl<H, P> Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
    fn try_decode_partial(&mut self, buf: &BytesMut, next: u8) -> Option<Frame<H, P>> {
        debug!("Attempting partial decoding");

        if self.expected_frames == 0 || self.expected_frames == 1 {
            return None;
        }

        let chunk_size = buf.len() / self.expected_frames;

        // Try to decode all partial frames from `self.expected_frames` < n <= 1.
        for n in (1..self.expected_frames).rev() {
            let mut chunks = buf.chunks(chunk_size);

            if let Ok(frame) = <Self as Decoder>::Item::from_ash_frames_buffered(
                (&mut chunks).take(n),
                &mut self.buffers.parameters,
            ) {
                // Check if the initially mismatched excess byte matches.
                // Otherwise, we probably received garbage.
                if let Some(chunk) = chunks.next() {
                    if chunk.first() == Some(&next) {
                        debug!("Successfully decoded frame from partial stream using {n} chunks of size {chunk_size}");
                        return Some(frame);
                    }
                }
            }
        }

        None
    }
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
            expected_frames: 0,
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
        match Self::Item::from_ash_frames_buffered(
            src.chunks(MAX_PAYLOAD_SIZE),
            &mut self.buffers.parameters,
        ) {
            Ok(frame) => {
                src.clear();
                self.header = None;
                Ok(Some(frame))
            }
            Err(Error::Decode(Decode::TooFewBytes)) => Ok(None),
            Err(Error::Decode(Decode::TooManyBytes { next })) => {
                self.try_decode_partial(src, next).map_or_else(
                    || Err(Error::Decode(Decode::TooManyBytes { next })),
                    |frame| Ok(Some(frame)),
                )
            }
            Err(other) => Err(other),
        }
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
            self.expected_frames = 1;
        } else {
            for (index, chunk) in self
                .buffers
                .parameters
                .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.buffers.header.len()))
                .enumerate()
            {
                dst.extend_from_slice(&self.buffers.header);
                dst.extend_from_slice(chunk);
                self.expected_frames = index.saturating_add(1);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct Buffers {
    pub header: heapless::Vec<u8, MAX_HEADER_SIZE>,
    pub parameters: heapless::Vec<u8, MAX_PARAMETER_SIZE>,
}

impl Buffers {
    /// Clears the buffers.
    pub fn clear(&mut self) {
        self.header.clear();
        self.parameters.clear();
    }
}
