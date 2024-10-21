use ashv2::Payload;
use le_stream::FromLeStream;

use crate::ashv2::parsable::Parsable;
use crate::error::Decode;
use crate::frame::{Extended, Handler, Header};
use crate::{Error, MAX_FRAME_SIZE};

#[derive(Debug)]
pub struct Stream<T>
where
    T: Iterator<Item = Payload>,
{
    frames: T,
    last_header: Option<Extended>,
    buffer: heapless::Vec<u8, MAX_FRAME_SIZE>,
}

impl<T> Stream<T>
where
    T: Iterator<Item = Payload>,
{
    #[must_use]
    pub const fn new(frames: T) -> Self {
        Self {
            frames,
            last_header: None,
            buffer: heapless::Vec::new(),
        }
    }

    fn next_frame(&mut self) -> Option<(Extended, impl Iterator<Item = u8>)> {
        let mut stream = self.frames.next()?.into_iter();
        Extended::from_le_stream(&mut stream).map(|header| (header, stream))
    }

    fn try_parse_buffer(&mut self, id: u16) -> Option<Result<Handler, Decode>> {
        match Handler::parse_from_le_stream(id, self.buffer.iter().copied()) {
            Ok(handler) => {
                self.buffer.clear();
                Some(Ok(handler))
            }
            Err(error) => {
                if error == Decode::TooFewBytes {
                    None
                } else {
                    self.buffer.clear();
                    Some(Err(error))
                }
            }
        }
    }
}

impl<T> Iterator for Stream<T>
where
    T: Iterator<Item = Payload>,
{
    type Item = Result<Handler, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(last_header) = self.last_header.take() {
                match self.try_parse_buffer(Header::<u16>::id(last_header)) {
                    Some(Ok(handler)) => return Some(Ok(handler)),
                    Some(Err(error)) => return Some(Err(error.into())),
                    None => {
                        if let Some((header, remainder)) = self.next_frame() {
                            self.last_header.replace(header);

                            if Header::<u16>::id(header) == Header::<u16>::id(last_header)
                                && Header::<u16>::sequence(header)
                                    == Header::<u16>::sequence(last_header)
                            {
                                self.buffer.extend(remainder);
                            } else {
                                self.buffer.clear();
                                self.buffer.extend(remainder);
                                return Some(Err(Decode::TooFewBytes.into()));
                            }
                        } else {
                            return Some(Err(Decode::TooFewBytes.into()));
                        }
                    }
                }
            } else if let Some((header, remainder)) = self.next_frame() {
                self.last_header = Some(header);
                self.buffer.extend(remainder);
            } else {
                return Some(Err(Decode::TooFewBytes.into()));
            }
        }
    }
}
