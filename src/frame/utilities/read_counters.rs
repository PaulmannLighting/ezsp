use crate::counter::Counter;
use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use never::Never;

const ID: u16 = 0x00F1;

/// Retrieves Ember counters.
///
/// See the [`Counter`] enumeration for the counter types.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
}

impl Command {
    #[must_use]
    pub const fn new(sequence: u8, control: Control) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
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

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    values: [u16; Counter::TypeCount as usize],
}

impl Response {
    #[must_use]
    pub const fn new(
        sequence: u8,
        control: Control,
        values: [u16; Counter::TypeCount as usize],
    ) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            values,
        }
    }

    #[must_use]
    pub const fn values(&self) -> &[u16; Counter::TypeCount as usize] {
        &self.values
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 * (Counter::TypeCount as usize));
        self.values
            .iter()
            .for_each(|value| parameters.extend_from_slice(&value.to_be_bytes()));
        Some(parameters)
    }
}
