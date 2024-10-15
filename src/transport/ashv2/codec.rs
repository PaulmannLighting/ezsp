use le_stream::{FromLeStream, ToLeStream};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::error::Decode;
use crate::frame::parameters::utilities::invalid_command;
use crate::frame::{Frame, Header, Parameter, ValidControl};
use crate::Error;

use ashv2::MAX_PAYLOAD_SIZE;

/// TODO: This is just an estimate. Check all frames and calculate actual maximum frame size.
const EZSP_MAX_FRAME_SIZE: usize = 256;
/// And `EZSP` header has a maximum size of 5 bytes.
const EZSP_MAX_HEADER_SIZE: usize = 5;

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::codec::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Codec<C, P>
where
    C: ValidControl,
    P: Parameter,
{
    header: Option<Header<C>>,
    _parameter: std::marker::PhantomData<P>,
    buffers: Buffers,
}

impl<C, P> Default for Codec<C, P>
where
    C: ValidControl,
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

impl<C, P> Decoder for Codec<C, P>
where
    C: ValidControl,
    P: Parameter + FromLeStream,
    <C as ValidControl>::Size: From<<P as Parameter>::Id>,
{
    type Item = Frame<C, P>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.buffers.clear();

        for chunk in src.chunks(MAX_PAYLOAD_SIZE) {
            let mut stream = chunk.iter().copied();

            let Some(header) = Header::<C>::from_le_stream(&mut stream) else {
                return Ok(None);
            };

            if header.id().into() == invalid_command::Response::ID {
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

        if item.header().id() == <C as ValidControl>::Size::from(<P as Parameter>::ID) {
            Ok(Some(item))
        } else {
            Err(Decode::FrameIdMismatch {
                expected: <P as Parameter>::ID.into(),
                found: item.header().id().into(),
            }
            .into())
        }
    }
}

impl<C, P> Encoder<Frame<C, P>> for Codec<C, P>
where
    C: ValidControl,
    P: Parameter + ToLeStream,
    <C as ValidControl>::Size: From<<P as Parameter>::Id>,
{
    type Error = Error;

    fn encode(&mut self, item: Frame<C, P>, dst: &mut BytesMut) -> Result<(), Self::Error> {
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
