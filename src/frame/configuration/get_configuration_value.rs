use crate::config;
use crate::frame::header::Header;
use crate::status::Status;

pub const ID: u8 = 0x52;

#[derive(Debug)]
pub struct Command {
    header: Header,
    config_id: config::Id,
}

impl Command {
    pub const fn new(header: Header, config_id: config::Id) -> Self {
        Self { header, config_id }
    }

    pub const fn config_id(&self) -> &config::Id {
        &self.config_id
    }
}

#[derive(Debug)]
pub struct Response {
    header: Header,
    status: Status,
    value: u16,
}

impl Response {
    pub const fn new(header: Header, status: Status, value: u16) -> Self {
        Self {
            header,
            status,
            value,
        }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }

    pub const fn value(&self) -> u16 {
        self.value
    }
}
