mod command;
mod response;

use command::Command;
pub use command::SleepMode;
use le_stream::{FromLeStream, ToLeStream};
pub use response::CallbackType;
use response::Response;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LowByte {
    Command(Command),
    Response(Response),
}

impl Default for LowByte {
    fn default() -> Self {
        Self::Command(Command::default())
    }
}

impl FromLeStream for LowByte {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let byte = u8::from_le_stream(bytes)?;

        if byte & Response::IS_RESPONSE.bits() != 0 {
            Some(Self::Response(Response::from_bits_retain(byte)))
        } else {
            Some(Self::Command(Command::from_bits_retain(byte)))
        }
    }
}

impl ToLeStream for LowByte {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Command(command) => command.to_le_stream(),
            Self::Response(response) => response.to_le_stream(),
        }
    }
}
