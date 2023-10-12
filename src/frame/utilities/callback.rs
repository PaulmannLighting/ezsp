use crate::frame::header::Header;
use crate::frame::Frame;
use never::Never;

const ID: u16 = 0x0006;

pub struct Command {
    header: Header,
}

impl Frame<ID> for Command {
    type Parameters = Never;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        None
    }
}
