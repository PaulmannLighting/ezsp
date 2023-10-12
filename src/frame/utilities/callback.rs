use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use never::Never;

const ID: u16 = 0x0006;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    pub const fn new(sequence: u8, control: Control) -> Self {
        Self {
            header: Self::make_header(sequence, control),
        }
    }
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
