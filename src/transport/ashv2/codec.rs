use crate::error::Decode;
use crate::frame::{Frame, Header, Parameter, ValidControl};
use ashv2::MAX_PAYLOAD_SIZE;
use le_stream::{FromLeStream, ToLeStream};
use tokio_util::bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

/// Codec to encode frames to bytes and decode bytes into frames.
///
/// This can be used with `tokio::codec::Framed` to encode and decode frames.
#[derive(Debug)]
pub struct Codec<C, P> {
    _control: std::marker::PhantomData<C>,
    _parameter: std::marker::PhantomData<P>,
}

impl<C, P> Default for Codec<C, P>
where
    C: ValidControl,
    P: Parameter,
    <P as Parameter>::Id: Into<C::Size>,
{
    fn default() -> Self {
        Self {
            _control: std::marker::PhantomData,
            _parameter: std::marker::PhantomData,
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
    type Error = crate::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut header: Option<Header<C>> = None;
        let mut parameters = Vec::new();

        for chunk in src.chunks(MAX_PAYLOAD_SIZE) {
            let mut stream = chunk.iter().copied();

            let Some(next_header) = Header::<C>::from_le_stream(&mut stream) else {
                return Ok(None);
            };

            if let Some(last_header) = header {
                if last_header.id() != next_header.id() {
                    return Err(Decode::FrameIdMismatch {
                        expected: last_header.id().into(),
                        found: next_header.id().into(),
                    }
                    .into());
                }
            } else {
                header.replace(next_header);
            }

            parameters.extend(stream);
        }

        let Some(header) = header else {
            return Ok(None);
        };

        match P::from_le_stream_exact(parameters.into_iter()) {
            Ok(parameters) => {
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
            Err(error) => match error {
                le_stream::Error::StreamNotExhausted(next) => {
                    Err(Decode::TooManyBytes { next }.into())
                }
                le_stream::Error::UnexpectedEndOfStream => Err(Decode::TooFewBytes.into()),
            },
        }
    }
}

impl<C, P> Encoder<Frame<C, P>> for Codec<C, P>
where
    C: ValidControl,
    P: Parameter + ToLeStream,
    <C as ValidControl>::Size: From<<P as Parameter>::Id>,
{
    type Error = crate::Error;

    fn encode(&mut self, item: Frame<C, P>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let header = item.header();
        let parameters: Vec<u8> = item.parameters().to_le_stream().collect();

        for chunk in
            parameters.chunks(MAX_PAYLOAD_SIZE.saturating_sub(header.to_le_stream().count()))
        {
            dst.extend(header.to_le_stream());
            dst.put_slice(chunk);
        }

        Ok(())
    }
}
