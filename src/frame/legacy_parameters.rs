use crate::frame::parameters::configuration::version::{Command, Response, ID};
use crate::read_write::Writable;
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
pub enum LegacyParameters {
    Command(Command),
    Response(Response),
}

impl LegacyParameters {
    #[must_use]
    pub const fn id(&self) -> u8 {
        ID
    }
}

impl From<Command> for LegacyParameters {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl From<Response> for LegacyParameters {
    fn from(response: Response) -> Self {
        Self::Response(response)
    }
}

impl Writable for LegacyParameters {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Command(command) => command.write_to(dst),
            Self::Response(response) => response.write_to(dst),
        }
    }
}
