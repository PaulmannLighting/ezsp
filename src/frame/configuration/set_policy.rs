use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use crate::{decision, policy};
use num_traits::ToPrimitive;

const ID: u16 = 0x0055;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    policy_id: policy::Id,
    decision_id: decision::Id,
}

impl Command {
    pub const fn new(header: Header, policy_id: policy::Id, decision_id: decision::Id) -> Self {
        Self {
            header,
            policy_id,
            decision_id,
        }
    }

    pub const fn policy_id(&self) -> &policy::Id {
        &self.policy_id
    }

    pub const fn decision_id(&self) -> &decision::Id {
        &self.decision_id
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![
            self.policy_id
                .to_u8()
                .expect("could not convert policy ID to u8"),
            self.decision_id
                .to_u8()
                .expect("could not convert decision ID to u8"),
        ]
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(header: Header, status: Status) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.status.to_u8().expect("could not convert status to u8")]
    }
}
