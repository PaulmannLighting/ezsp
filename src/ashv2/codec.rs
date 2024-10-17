use le_stream::ToLeStream;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::constants::{EZSP_MAX_FRAME_SIZE, EZSP_MAX_HEADER_SIZE};
use crate::error::Decode;
use crate::frame::{Frame, Header, Parameter};
use crate::Error;

use crate::ashv2::parsable::Parsable;
use ashv2::MAX_PAYLOAD_SIZE;

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
            dst.extend_from_slice(&self.buffers.header);
        } else {
            for chunk in self
                .buffers
                .parameters
                .chunks(MAX_PAYLOAD_SIZE.saturating_sub(self.buffers.header.len()))
            {
                dst.extend_from_slice(&self.buffers.header);
                dst.extend_from_slice(chunk);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct Buffers {
    pub header: heapless::Vec<u8, EZSP_MAX_HEADER_SIZE>,
    pub parameters: heapless::Vec<u8, EZSP_MAX_FRAME_SIZE>,
}

impl Buffers {
    /// Clears the buffers.
    pub fn clear(&mut self) {
        self.header.clear();
        self.parameters.clear();
    }
}
