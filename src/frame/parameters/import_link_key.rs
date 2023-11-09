use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{sl_zb_sec_man_key_t,sl_status_t,EmberEUI64};

pub const ID: u16 = 0x010E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    index: u8,
    address: EmberEUI64,
    plaintext_key: sl_zb_sec_man_key_t,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, address: EmberEUI64, plaintext_key: sl_zb_sec_man_key_t) -> Self {
        Self { index, address, plaintext_key }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }


    #[must_use]
    pub const fn address(&self) -> EmberEUI64 {
        self.address
    }


    #[must_use]
    pub const fn plaintext_key(&self) -> sl_zb_sec_man_key_t {
        self.plaintext_key
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
