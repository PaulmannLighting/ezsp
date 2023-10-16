use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use crate::{decision, policy};
use std::io::Read;

const ID: u16 = 0x0055;

/// Allows the Host to change the policies used by the NCP to make fast decisions.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    policy_id: policy::Id,
    decision_id: decision::Id,
}

impl Command {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        policy_id: policy::Id,
        decision_id: decision::Id,
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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

impl Frame<ID> for Command {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.policy_id.into(), self.decision_id.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [policy_id, decision_id]: [u8; 2] = [0; 2];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            policy_id: policy::Id::try_from(policy_id)?,
            decision_id: decision::Id::try_from(decision_id)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.into()])
    }

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let header = Self::read_header(src)?;
        let mut buffer @ [status]: [u8; 1] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            header,
            status: Status::try_from(status)?,
        })
    }
}
