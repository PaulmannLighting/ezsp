use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::mfg_token;
use num_traits::ToPrimitive;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x000B;

/// Retrieves a manufacturing token from the Flash Information Area of the NCP
/// (except for EZSP_STACK_CAL_DATA which is managed by the stack).
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: mfg_token::Id,
}

impl Command {
    pub const fn new(sequence: u8, control: Control, token_id: mfg_token::Id) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_id,
        }
    }

    pub const fn token_id(&self) -> &mfg_token::Id {
        &self.token_id
    }
}

impl Frame<ID> for Command {
    type Parameters = [u8; 1];

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self
            .token_id
            .to_u8()
            .expect("could not convert token ID to u8")])
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Response {
    pub fn new(
        sequence: u8,
        control: Control,
        token_data: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_data_length: token_data.len().try_into()?,
            token_data,
        })
    }

    pub const fn token_data_length(&self) -> u8 {
        self.token_data_length
    }

    pub fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl Frame<ID> for Response {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(1 + self.token_data.len());
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
    }
}
