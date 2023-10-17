use crate::ezsp::Status;
use crate::frame::Parameters;
use crate::{decision, policy};
use std::array::IntoIter;
use std::io::Read;

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
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.policy_id.into(), self.decision_id.into()].into_iter()
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [policy_id, decision_id] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            policy_id: policy::Id::try_from(policy_id)?,
            decision_id: decision::Id::try_from(decision_id)?,
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
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
