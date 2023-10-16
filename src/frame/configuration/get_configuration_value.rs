use crate::config;
use crate::frame::Parameters;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::io::Read;

pub const ID: u16 = 0x0052;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    config_id: config::Id,
}

/// Reads a configuration value from the NCP.
impl Command {
    #[must_use]
    pub const fn new(config_id: config::Id) -> Self {
        Self { config_id }
    }

    #[must_use]
    pub const fn config_id(&self) -> config::Id {
        self.config_id
    }
}

impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        vec![command
            .config_id
            .to_u8()
            .expect("could not convert config id to u8")]
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [config_id] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            config_id: config::Id::try_from(config_id)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    value: u16,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, value: u16) -> Self {
        Self { status, value }
    }

    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl From<Response> for Vec<u8> {
    fn from(response: Response) -> Self {
        let [value_low, value_high] = response.value.to_be_bytes();
        vec![
            response
                .status
                .to_u8()
                .expect("could not convert status to u8"),
            value_low,
            value_high,
        ]
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status, value @ ..] = [0; 3];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
            value: u16::from_be_bytes(value),
        })
    }
}
