use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::status::Status;
use crate::value;
use num_traits::ToPrimitive;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x003;

/// Reads a value from the NCP but passes an extra argument specific to the value being retrieved.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    value_id: value::ExtendedId,
    characteristics: u32,
}

impl Command {
    pub const fn new(
        sequence: u8,
        control: Control,
        value_id: value::ExtendedId,
        characteristics: u32,
    ) -> Self {
        Self {
            header: Self::make_header(sequence, control),
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

    fn parameters(&self) -> Option<Self::Parameters> {
        let [characteristics_low, characteristics_high] = self.characteristics.to_be_bytes();
        Some([
            self.value_id
                .to_u8()
                .expect("could not convert value ID to u8"),
            characteristics_low,
            characteristics_high,
        ])
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
    value_length: u8,
    value: Arc<[u8]>,
}

impl Response {
    pub fn new(
        sequence: u8,
        control: Control,
        status: Status,
        value: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Self::make_header(sequence, control),
            status,
            value_length: value.len().try_into()?,
            value,
        })
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }

    pub const fn value_length(&self) -> u8 {
        self.value_length
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

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.value.len());
        parameters.push(self.status.to_u8().expect("could not convert status to u8"));
        parameters.push(self.value_length);
        parameters.extend_from_slice(&self.value);
        Some(parameters)
    }
}
