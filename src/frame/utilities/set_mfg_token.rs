use crate::frame::header::{Control, Header};
use crate::frame::Frame;
use crate::mfg_token;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::num::TryFromIntError;
use std::sync::Arc;

const ID: u16 = 0x000C;

/// Sets a manufacturing token in the Customer Information Block (CIB)
/// area of the NCP if that token currently unset (fully erased).
///
/// Cannot be used with EZSP_STACK_CAL_DATA, EZSP_STACK_CAL_FILTER,
/// EZSP_MFG_ASH_CONFIG, or EZSP_MFG_CBKE_DATA token.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: mfg_token::Id,
    token_data_length: u8,
    token_data: Arc<[u8]>,
}

impl Command {
    pub fn new(
        sequence: u8,
        control: Control,
        token_id: mfg_token::Id,
        token_data: Arc<[u8]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            header: Header::for_frame::<ID>(sequence, control),
            token_id,
            token_data_length: token_data.len().try_into()?,
            token_data,
        })
    }

    pub const fn token_id(&self) -> &mfg_token::Id {
        &self.token_id
    }

    pub const fn token_data_length(&self) -> u8 {
        self.token_data_length
    }

    pub const fn token_data(&self) -> &[u8] {
        &self.token_data
    }
}

impl Frame<ID> for Command {
    type Parameters = Vec<u8>;

    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Option<Self::Parameters> {
        let mut parameters = Vec::with_capacity(2 + self.token_data.len());
        parameters.push(
            self.token_id
                .to_u8()
                .expect("could not convert token ID to u8"),
        );
        parameters.push(self.token_data_length);
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(sequence: u8, control: Control, status: Status) -> Self {
        Self {
            header: Header::for_frame::<ID>(sequence, control),
            status,
        }
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

    fn parameters(&self) -> Option<Self::Parameters> {
        Some([self.status.to_u8().expect("could not convert status to u8")])
    }
}
