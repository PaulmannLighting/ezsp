use crate::types::{sl_status_t, sl_zb_sec_man_network_key_info_t};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0116;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: sl_status_t,
    network_key_info: sl_zb_sec_man_network_key_info_t,
}

impl Response {
    #[must_use]
    pub const fn new(
        status: sl_status_t,
        network_key_info: sl_zb_sec_man_network_key_info_t,
    ) -> Self {
        Self {
            status,
            network_key_info,
        }
    }

    #[must_use]
    pub const fn status(&self) -> sl_status_t {
        self.status
    }

    #[must_use]
    pub const fn network_key_info(&self) -> sl_zb_sec_man_network_key_info_t {
        self.network_key_info
    }
}
