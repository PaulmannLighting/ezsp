use le_stream::{FromLeStream, ToLeStream};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::constants::{EZSP_MAX_FRAME_SIZE, EZSP_MAX_HEADER_SIZE};
use crate::error::Decode;
use crate::frame::parameters::utilities::invalid_command;
use crate::frame::{Frame, Header, Parameter};
use crate::Error;

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
    P: Parameter + FromLeStream,
{
    type Item = Frame<H, P>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.buffers.clear();

        for chunk in src.chunks(MAX_PAYLOAD_SIZE) {
            let mut stream = chunk.iter().copied();

            let Some(header) = H::from_le_stream(&mut stream) else {
                return Ok(None);
            };

            if header.id().into()
                == invalid_command::Response::ID.ok_or_else(|| Error::from(Decode::MissingId))?
            {
                return Err(Error::InvalidCommand(
                    invalid_command::Response::from_le_stream_exact(stream)?,
                ));
            }

            if let Some(current_header) = self.header {
                if current_header.id() != header.id() {
                    return Err(Decode::FrameIdMismatch {
                        expected: current_header.id().into(),
                        found: header.id().into(),
                    }
                    .into());
                }
            } else {
                self.header.replace(header);
            }

            self.buffers.parameters.extend(stream);
        }

        let Some(header) = self.header else {
            return Ok(None);
        };

        let parameters = P::from_le_stream_exact(self.buffers.parameters.iter().copied())?;
        src.clear();
        let item = Self::Item::new(header, parameters);

        if let Some(id) = P::ID {
            if item.header().id() == id {
                Ok(Some(item))
            } else {
                Err(Decode::FrameIdMismatch {
                    expected: id.into(),
                    found: item.header().id().into(),
                }
                .into())
            }
        } else {
            Ok(Some(item))
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
