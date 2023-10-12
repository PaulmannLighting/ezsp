use crate::frame::header::Header;
use crate::frame::Frame;
use never::Never;

const ID: u16 = 0x0007;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
}

impl Response {
    pub const fn new(header: Header) -> Self {
        Self { header }
    }
}

impl Frame<ID> for Response {
    type Parameters = Never;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        None
    }
}
