use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0058;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    reason: Status,
}

impl Response {
    pub const fn new(header: Header, reason: Status) -> Self {
        Self { header, reason }
    }

    pub const fn reason(&self) -> &Status {
        &self.reason
    }
}

impl Frame<ID> for Response {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        [self.reason.to_u8().expect("could not convert reason to u8")]
    }
}
