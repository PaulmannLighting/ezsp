use crate::types::bool;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00D8;

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
    zll_rx_on_when_idle_get_active: bool,
}

impl Response {
    #[must_use]
    pub const fn new(zll_rx_on_when_idle_get_active: bool) -> Self {
        Self {
            zll_rx_on_when_idle_get_active,
        }
    }

    #[must_use]
    pub const fn zll_rx_on_when_idle_get_active(&self) -> bool {
        self.zll_rx_on_when_idle_get_active
    }
}
