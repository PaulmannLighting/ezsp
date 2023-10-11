use crate::config;
use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;

const ID: u16 = 0x0053;

#[derive(Debug)]
pub struct Command {
    header: Header,
    config_id: config::Id,
    value: u16,
}

impl Command {
    pub const fn new(header: Header, config_id: config::Id, value: u16) -> Self {
        Self {
            header,
            config_id,
            value,
        }
    }

    pub const fn config_id(&self) -> &config::Id {
        &self.config_id
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}

impl Frame<ID, 3> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> [u8; 3] {
        let [high, low] = self.value.to_be_bytes();
        [
            self.config_id
                .to_u8()
                .expect("could not convert config_id to u8"),
            high,
            low,
        ]
    }
}

#[derive(Debug)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(header: Header, status: Status) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID, 1> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> [u8; 1] {
        [self.status.to_u8().expect("could not convert status to u8")]
    }
}
