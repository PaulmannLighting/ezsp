mod command;
mod response;

use std::array::IntoIter;

pub use command::{Command, SleepMode};
use le_stream::{FromLeStream, ToLeStream};
pub use response::{CallbackType, Response};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LowByte {
    Command(Command),
    Response(Response),
}

impl LowByte {
    #[must_use]
    pub const fn is_command(self) -> bool {
        matches!(self, Self::Command(_))
    }

    #[must_use]
    pub const fn is_response(self) -> bool {
        matches!(self, Self::Response(_))
    }
}

impl From<Command> for LowByte {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl From<Response> for LowByte {
    fn from(response: Response) -> Self {
        Self::Response(response)
    }
}

impl FromLeStream for LowByte {
    fn from_le_stream<T>(stream: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let byte = u8::from_le_stream(stream)?;

        if byte & Command::IS_RESPONSE.bits() == 0 {
            Some(Self::Command(Command::from_bits_truncate(byte)))
        } else {
            Some(Self::Response(Response::from_bits_truncate(byte)))
        }
    }
}

impl ToLeStream for LowByte {
    type Iter = IntoIter<u8, 1>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Command(command) => command.bits().to_le_stream(),
            Self::Response(response) => response.bits().to_le_stream(),
        }
    }
}
