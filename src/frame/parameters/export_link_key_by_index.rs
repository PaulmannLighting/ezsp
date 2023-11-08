use crate::types::{
    sl_status_t, sl_zb_sec_man_aps_key_metadata_t, sl_zb_sec_man_key_t, EmberEUI64,
};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x010F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    eui: EmberEUI64,
    plaintext_key: sl_zb_sec_man_key_t,
    key_data: sl_zb_sec_man_aps_key_metadata_t,
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(
        eui: EmberEUI64,
        plaintext_key: sl_zb_sec_man_key_t,
        key_data: sl_zb_sec_man_aps_key_metadata_t,
        status: sl_status_t,
    ) -> Self {
        Self {
            eui,
            plaintext_key,
            key_data,
            status,
        }
    }

    #[must_use]
    pub const fn eui(&self) -> EmberEUI64 {
        self.eui
    }

    #[must_use]
    pub const fn plaintext_key(&self) -> sl_zb_sec_man_key_t {
        self.plaintext_key
    }

    #[must_use]
    pub const fn key_data(&self) -> sl_zb_sec_man_aps_key_metadata_t {
        self.key_data
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
