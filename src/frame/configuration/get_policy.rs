use crate::ezsp::Status;
use crate::frame::Parameters;
use crate::policy;
use crate::util::ReadExt;
use std::io::Read;
use std::iter::{once, Empty, Once};

pub const ID: u16 = 0x0056;

/// Allows the Host to read the policies used by the NCP to make fast decisions.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    policy_id: policy::Id,
}

impl Command {
    #[must_use]
    pub const fn new(policy_id: policy::Id) -> Self {
        Self { policy_id }
    }

    #[must_use]
    pub const fn policy_id(&self) -> policy::Id {
        self.policy_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            policy_id: src.read_u8()?.try_into()?,
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            status: src.read_u8()?.try_into()?,
        })
    }
}
