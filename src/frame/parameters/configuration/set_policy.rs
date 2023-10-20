use crate::ezsp::Status;
use crate::read_write::Readable;
use crate::{decision, policy};
use rw_exact_ext::ReadExactExt;
use std::io::Read;
use std::iter::{once, Chain, Once};

pub const ID: u16 = 0x0055;

/// Allows the Host to change the policies used by the NCP to make fast decisions.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    policy_id: policy::Id,
    decision_id: decision::Id,
}

impl Command {
    #[must_use]
    pub const fn new(policy_id: policy::Id, decision_id: decision::Id) -> Self {
        Self {
            policy_id,
            decision_id,
        }
    }

    #[must_use]
    pub const fn policy_id(&self) -> policy::Id {
        self.policy_id
    }

    #[must_use]
    pub const fn decision_id(&self) -> decision::Id {
        self.decision_id
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<Once<Self::Item>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.policy_id.into()).chain(once(self.decision_id.into()))
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let [policy_id, decision_id] = src.read_array_exact()?;
        Ok(Self {
            policy_id: policy_id.try_into()?,
            decision_id: decision_id.try_into()?,
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
