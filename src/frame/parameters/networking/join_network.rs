use crate::ember::node::Type;
use crate::ember::{network, Status};
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x001F;

/// Causes the stack to associate with the network using the specified network parameters.
///
/// It can take several seconds for the stack to associate with the local network.
/// Do not send messages until the stackStatusHandler callback informs you that the stack is up.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    note_type: Type,
    parameters: network::Parameters,
}

impl Command {
    #[must_use]
    pub const fn node_type(&self) -> Type {
        self.note_type
    }

    #[must_use]
    pub const fn parameters(&self) -> &network::Parameters {
        &self.parameters
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, <network::Parameters as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.note_type.into()).chain(self.parameters)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let node_type: u8 = src.read_num_be()?;
        let parameters = network::Parameters::read_from(src)?;
        Ok(Self {
            note_type: node_type.try_into()?,
            parameters,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
