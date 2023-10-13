use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use crate::{decision, policy};
use num_traits::ToPrimitive;

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
    pub const fn policy_id(&self) -> &policy::Id {
        &self.policy_id
    }

    #[must_use]
    pub const fn decision_id(&self) -> &decision::Id {
        &self.decision_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 2];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([
            self.policy_id
                .to_u8()
                .expect("could not convert policy ID to u8"),
            self.decision_id
                .to_u8()
                .expect("could not convert decision ID to u8"),
        ])
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
    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.to_u8().expect("could not convert status to u8")])
    }
}
