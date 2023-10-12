use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use crate::value;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x00AA;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    value_id: value::Id,
}

impl Command {
    pub const fn new(header: Header, value_id: value::Id) -> Self {
        Self { header, value_id }
    }

    pub const fn value_id(&self) -> &value::Id {
        &self.value_id
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self
            .value_id
            .to_u8()
            .expect("could not convert value ID to u8")]
    }
}

pub struct Response {
    header: Header,
    status: Status,
    value: Arc<[u8]>,
}

impl Response {
    pub const fn new(header: Header, status: Status, value: Arc<[u8]>) -> Self {
        Self {
            header,
            status,
            value,
        }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }

    pub const fn value_length(&self) -> u8 {
        self.value
            .len()
            .try_into()
            .expect("value length exceeds u8")
    }

    pub const fn value(&self) -> &[u8] {
        &self.value
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.status.to_u8().expect("could not convert status to u8"));
        parameters.push(self.value_length());
        parameters.extend_from_slice(&self.value);
        parameters
    }
}
