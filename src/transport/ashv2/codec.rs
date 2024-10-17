use le_stream::{FromLeStream, ToLeStream};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::constants::{EZSP_MAX_FRAME_SIZE, EZSP_MAX_HEADER_SIZE};
use crate::error::Decode;
use crate::frame::parameters::utilities::invalid_command;
use crate::frame::{Frame, Header, Parameter};
use crate::Error;

use crate::parsable::Parsable;
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

impl<H, P> Codec<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    fn process_chunk(&mut self, chunk: &[u8]) -> Result<bool, Error> {
        let mut stream = chunk.iter().copied();

        let Some(header) = H::from_le_stream(&mut stream) else {
            return Ok(false);
        };

        if header.id().into() == invalid_command::Response::ID {
            return Err(Error::InvalidCommand(
                invalid_command::Response::from_le_stream_exact(stream)?,
            ));
        }

        if let Some(last_header) = self.header {
            if last_header.id() != header.id() {
                return Err(Decode::FrameIdMismatch {
                    expected: last_header.id().into(),
                    found: header.id().into(),
                }
                .into());
            }
        } else {
            self.header.replace(header);
        }

        self.buffers.parameters.extend(stream);
        Ok(true)
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

        for chunk in src.chunks(MAX_PAYLOAD_SIZE) {
            if !self.process_chunk(chunk)? {
                return Ok(None);
            }
        }

        let Some(header) = self.header else {
            return Ok(None);
        };

        let parameters =
            P::parse_from_le_stream(header.id().into(), self.buffers.parameters.iter().copied())?;
        src.clear();
        Ok(Some(Self::Item::new(header, parameters)))
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
