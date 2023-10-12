use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use crate::value;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x003;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    value_id: value::ExtendedId,
    characteristics: u32,
}

impl Command {
    pub const fn new(header: Header, value_id: value::ExtendedId, characteristics: u32) -> Self {
        Self {
            header,
            value_id,
            characteristics,
        }
    }

    pub const fn value_id(&self) -> &value::ExtendedId {
        &self.value_id
    }

    pub const fn characteristics(&self) -> u32 {
        self.characteristics
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 3];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        let [characteristics_low, characteristics_high] = self.characteristics.to_be_bytes();
        [
            self.value_id
                .to_u8()
                .expect("could not convert value ID to u8"),
            characteristics_low,
            characteristics_high,
        ]
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Self::Parameters {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.status.to_u8().expect("could not convert status to u8"));
        parameters.push(self.value_length());
        parameters.extend_from_slice(&self.value);
        parameters
    }
}
