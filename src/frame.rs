use std::fmt::Debug;

use crate::parameters::utilities::invalid_command;
use crate::Error;
#[cfg(feature = "ashv2")]
use crate::{constants::EZSP_MAX_FRAME_SIZE, error::Decode, parsable::Parsable};
pub use handler::Handler;
pub use header::{Command, Extended, Header, Legacy};
use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::FromLeStream;
pub use parameter::Parameter;

mod handler;
mod header;
mod parameter;
pub mod parameters;

/// A frame that contains a header and parameters.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    header: H,
    parameters: P,
}

impl<H, P> Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter,
{
    /// Create a new frame.
    #[must_use]
    pub const fn new(header: H, parameters: P) -> Self {
        Self { header, parameters }
    }

    /// Return the header.
    #[must_use]
    pub const fn header(&self) -> H {
        self.header
    }

    /// Return the parameters.
    pub fn parameters(self) -> P {
        self.parameters
    }
}

#[cfg(feature = "ashv2")]
impl<H, P> Frame<H, P>
where
    H: Header<P::Id>,
    P: Parameter + Parsable,
{
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
