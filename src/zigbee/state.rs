//! Zigbee network manager state.

use std::collections::HashMap;

use crate::ezsp::{config, decision, policy};
use crate::parameters::bootloader::get_standalone_bootloader_version_plat_micro_phy;

#[derive(Debug)]
pub struct State {
    bootloader_version: get_standalone_bootloader_version_plat_micro_phy::Response,
    configuration: HashMap<config::Id, u16>,
    policies: HashMap<policy::Id, decision::Id>,
}

impl State {
    /// Creates a new state.
    #[must_use]
    pub const fn new(
        bootloader_version: get_standalone_bootloader_version_plat_micro_phy::Response,
        configuration: HashMap<config::Id, u16>,
        policies: HashMap<policy::Id, decision::Id>,
    ) -> Self {
        Self {
            bootloader_version,
            configuration,
            policies,
        }
    }

    /// Returns the bootloader version.
    #[must_use]
    pub const fn bootloader_version(
        &self,
    ) -> &get_standalone_bootloader_version_plat_micro_phy::Response {
        &self.bootloader_version
    }

    /// Returns the configuration.
    #[must_use]
    pub const fn configuration(&self) -> &HashMap<config::Id, u16> {
        &self.configuration
    }

    /// Returns the policies.
    #[must_use]
    pub const fn policies(&self) -> &HashMap<policy::Id, decision::Id> {
        &self.policies
    }
}
