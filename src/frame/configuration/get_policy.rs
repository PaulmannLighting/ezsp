use crate::frame::header::Header;
use crate::frame::Frame;
use crate::policy;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0056;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    policy_id: policy::Id,
}

impl Command {
    pub const fn new(header: Header, policy_id: policy::Id) -> Self {
        Self { header, policy_id }
    }

    pub const fn policy_id(&self) -> &policy::Id {
        &self.policy_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        [self
            .policy_id
            .to_u8()
            .expect("could not convert policy ID to u8")]
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
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        [self.status.to_u8().expect("could not convert status to u8")]
    }
}
