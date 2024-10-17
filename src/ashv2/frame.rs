use crate::ashv2::Parsable;
use crate::constants::EZSP_MAX_FRAME_SIZE;
use crate::error::Decode;
use crate::frame::{Frame, Header, Parameter};
use crate::parameters::utilities::invalid_command;
use crate::Error;
use le_stream::FromLeStream;

/// Extension of [`Frame`] to support parsing it from an iterator of `ASHv2` frames.
impl<H, P> Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
    /// Parse a frame from an iterator of `ASHv2` frames.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the frame ID does not match the expected ID
    /// or if the frame is an [`invalid_command::Response`].
    pub fn from_ash_frames_buffered<'a, T>(
        frames: T,
        parameters: &mut heapless::Vec<u8, EZSP_MAX_FRAME_SIZE>,
    ) -> crate::Result<Self>
    where
        T: Iterator<Item = &'a [u8]>,
    {
        parameters.clear();
        let mut header: Option<H> = None;

        for frame in frames {
            let mut stream = frame.iter().copied();
            let next_header = H::from_le_stream(&mut stream).ok_or(Decode::TooFewBytes)?;

            if let Some(header) = header {
                if header.id() != next_header.id() || header.sequence() != next_header.sequence() {
                    return Err(Decode::FrameIdMismatch {
                        expected: header.id().into(),
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
            return Err(Decode::TooFewBytes.into());
        };

        if header.id().into() == invalid_command::Response::ID {
            return Err(Error::InvalidCommand(
                invalid_command::Response::from_le_stream_exact(parameters.iter().copied())?,
            ));
        }

        Ok(Self::new(
            header,
            P::parse_from_le_stream(header.id().into(), parameters.iter().copied())?,
        ))
    }
}
