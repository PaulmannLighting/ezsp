use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::read_write::Readable;
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x00FD;

/// Returns the current radio parameters based on phy index.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    child_count: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_count: u8) -> Self {
        Self { child_count }
    }

    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.child_count)
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            child_count: src.read_num_be()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
    node_type: Type,
    parameters: Parameters,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, node_type: Type, parameters: Parameters) -> Self {
        Self {
            status,
            node_type,
            parameters,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub const fn node_type(&self) -> Type {
        self.node_type
    }

    #[must_use]
    pub const fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter =
        Chain<Chain<Once<Self::Item>, Once<Self::Item>>, <Parameters as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
            .chain(once(self.node_type.into()))
            .chain(self.parameters)
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_be()?;
        let node_type: u8 = src.read_num_be()?;
        let parameters = Parameters::read_from(src)?;
        Ok(Self {
            status: status.try_into()?,
            node_type: node_type.try_into()?,
            parameters,
        })
    }
}
