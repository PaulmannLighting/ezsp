use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

/// An example decoder.
#[derive(Debug)]
pub struct RawCodec;

impl Decoder for RawCodec {
    type Item = Box<[u8]>;
    type Error = std::io::Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buffer.len() >= 4 {
            Ok(Some(buffer.split().as_ref().into()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Box<[u8]>> for RawCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Box<[u8]>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item);
        Ok(())
    }
}
