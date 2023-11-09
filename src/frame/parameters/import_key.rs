use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{sl_zb_sec_man_key_t,sl_zb_sec_man_context_t,sl_status_t};

pub const ID: u16 = 0x0115;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    context: sl_zb_sec_man_context_t,
    key: sl_zb_sec_man_key_t,
}

impl Command {
    #[must_use]
    pub const fn new(context: sl_zb_sec_man_context_t, key: sl_zb_sec_man_key_t) -> Self {
        Self { context, key }
    }

    #[must_use]
    pub const fn context(&self) -> sl_zb_sec_man_context_t {
        self.context
    }


    #[must_use]
    pub const fn key(&self) -> sl_zb_sec_man_key_t {
        self.key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(status: sl_status_t) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
