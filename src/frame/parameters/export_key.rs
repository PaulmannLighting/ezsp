use crate::types::{sl_status_t, sl_zb_sec_man_context_t, sl_zb_sec_man_key_t};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0114;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    context: sl_zb_sec_man_context_t,
}

impl Command {
    #[must_use]
    pub const fn new(context: sl_zb_sec_man_context_t) -> Self {
        Self { context }
    }

    #[must_use]
    pub const fn context(&self) -> sl_zb_sec_man_context_t {
        self.context
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    key: sl_zb_sec_man_key_t,
    status: sl_status_t,
}

impl Response {
    #[must_use]
    pub const fn new(key: sl_zb_sec_man_key_t, status: sl_status_t) -> Self {
        Self { key, status }
    }

    #[must_use]
    pub const fn key(&self) -> sl_zb_sec_man_key_t {
        self.key
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }
}
