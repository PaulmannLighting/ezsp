use crate::frame::header::Header;
use crate::frame::Frame;
use crate::mfg_token;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x000B;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    token_id: mfg_token::Id,
}

impl Command {
    pub const fn new(header: Header, token_id: mfg_token::Id) -> Self {
        Self { header, token_id }
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
    token_data: Arc<[u8]>,
}

impl Response {
    pub const fn new(header: Header, token_data: Arc<[u8]>) -> Self {
        Self { header, token_data }
    }

    pub const fn token_data_length(&self) -> u8 {
        self.token_data
            .len()
            .try_into()
            .expect("token data length exceeds u8")
    }

    pub const fn token_data(&self) -> &[u8] {
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
        parameters.push(self.token_data_length());
        parameters.extend_from_slice(&self.token_data);
        Some(parameters)
    }
}
