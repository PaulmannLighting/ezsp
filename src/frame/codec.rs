use crate::frame::{Frame, Header, Parameter, ValidControl};
use le_stream::{FromLeStream, ToLeStream};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

/// A frame codec.
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
    <P as Parameter>::Id: Into<C::Size>,
{
    type Item = Frame<C, P>;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut stream = src.iter().copied();

        let Some(header) = Header::<C>::from_le_stream(&mut stream) else {
            return Ok(None);
        };

        match P::from_le_stream_exact(stream) {
            Ok(parameters) => {
                src.clear();
                Ok(Some(Self::Item::new(header, parameters)))
            }
            Err(error) => match error {
                le_stream::Error::StreamNotExhausted(next) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Too many bytes for frame. Next excess byte: {next:#04X}"),
                )),
                le_stream::Error::UnexpectedEndOfStream => Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    format!("Not enough bytes for frame: {:#04X?}", src.as_ref()),
                )),
            },
        }
    }
}

impl<C, P> Encoder<Frame<C, P>> for Codec<C, P>
where
    C: ValidControl,
    P: Parameter + ToLeStream,
    <P as Parameter>::Id: Into<C::Size>,
{
    type Error = std::io::Error;

    fn encode(&mut self, item: Frame<C, P>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend(item.header.to_le_stream());
        dst.extend(item.parameters.to_le_stream());
        Ok(())
    }
}
