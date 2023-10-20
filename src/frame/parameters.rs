use crate::read_write::Writable;
use std::io::Write;

pub mod configuration;
pub mod networking;
pub mod utilities;

#[derive(Debug, Eq, PartialEq)]
pub enum Parameters {
    Command(Command),
    Response(Response),
    Callback(Callback),
}

impl Parameters {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Command(command) => command.id(),
            Self::Response(response) => response.id(),
            Self::Callback(callback) => callback.id(),
        }
    }
}

impl From<Command> for Parameters {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl From<Response> for Parameters {
    fn from(response: Response) -> Self {
        Self::Response(response)
    }
}

impl Writable for Parameters {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Command(command) => command.write_to(dst),
            Self::Response(response) => response.write_to(dst),
            Self::Callback(callback) => callback.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Configuration(configuration::Command),
    Networking(networking::Command),
    Utilities(utilities::Command),
}

impl Command {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Configuration(configuration) => configuration.id(),
            Self::Networking(networking) => networking.id(),
            Self::Utilities(utilities) => utilities.id(),
        }
    }
}

impl From<configuration::Command> for Command {
    fn from(configuration: configuration::Command) -> Self {
        Self::Configuration(configuration)
    }
}

impl From<networking::Command> for Command {
    fn from(networking: networking::Command) -> Self {
        Self::Networking(networking)
    }
}

impl From<utilities::Command> for Command {
    fn from(utilities: utilities::Command) -> Self {
        Self::Utilities(utilities)
    }
}

impl Writable for Command {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Configuration(configuration) => configuration.write_to(dst),
            Self::Networking(networking) => networking.write_to(dst),
            Self::Utilities(utilities) => utilities.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    Configuration(configuration::Response),
    Networking(networking::Response),
    Utilities(utilities::Response),
}

impl Response {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Configuration(configuration) => configuration.id(),
            Self::Networking(networking) => networking.id(),
            Self::Utilities(utilities) => utilities.id(),
        }
    }
}

impl From<configuration::Response> for Response {
    fn from(configuration: configuration::Response) -> Self {
        Self::Configuration(configuration)
    }
}

impl From<networking::Response> for Response {
    fn from(networking: networking::Response) -> Self {
        Self::Networking(networking)
    }
}

impl From<utilities::Response> for Response {
    fn from(utilities: utilities::Response) -> Self {
        Self::Utilities(utilities)
    }
}

impl Writable for Response {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Configuration(configuration) => configuration.write_to(dst),
            Self::Networking(networking) => networking.write_to(dst),
            Self::Utilities(utilities) => utilities.write_to(dst),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Callback {
    Networking(networking::Callback),
    Utilities(utilities::Callback),
}

impl Callback {
    #[must_use]
    pub const fn id(&self) -> u16 {
        match self {
            Self::Networking(networking) => networking.id(),
            Self::Utilities(utilities) => utilities.id(),
        }
    }
}

impl Writable for Callback {
    fn write_to<W>(self, dst: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            Self::Networking(networking) => networking.write_to(dst),
            Self::Utilities(utilities) => utilities.write_to(dst),
        }
    }
}
