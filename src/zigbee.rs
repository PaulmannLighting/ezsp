//! Zigbee network manager implementation.

use std::collections::HashMap;

use enum_iterator::all;

use crate::ezsp::{config, decision, policy};
use crate::zigbee::state::State;
use crate::{Error, Ezsp};

mod state;

/// Zigbee network manager using an arbitrary `Ezsp` implementation.
#[derive(Debug)]
pub struct EzspNetworkManager<T>
where
    T: Ezsp,
{
    ezsp: T,
    state: Option<State>,
}

impl<T> EzspNetworkManager<T>
where
    T: Ezsp + Send,
{
    /// Creates a new network manager.
    #[must_use]
    pub const fn new(ezsp: T) -> Self {
        Self { ezsp, state: None }
    }

    /// Initializes the network manager.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the initialization fails.
    pub async fn init(&mut self) -> Result<(), Error> {
        Ezsp::init(&mut self.ezsp).await?;
        let bootloader_version = self
            .ezsp
            .get_standalone_bootloader_version_plat_micro_phy()
            .await?;
        let configuration = self.get_configuration_values().await?;
        let policies = self.get_policies().await?;
        self.state
            .replace(State::new(bootloader_version, configuration, policies));
        Ok(())
    }

    async fn get_configuration_values(&mut self) -> Result<HashMap<config::Id, u16>, Error> {
        let mut configuration = HashMap::default();

        for id in all::<config::Id>() {
            configuration.insert(id, self.ezsp.get_configuration_value(id).await?);
        }

        Ok(configuration)
    }

    async fn get_policies(&mut self) -> Result<HashMap<policy::Id, decision::Id>, Error> {
        let mut policies = HashMap::default();

        for id in all::<policy::Id>() {
            policies.insert(id, self.ezsp.get_policy(id).await?);
        }

        Ok(policies)
    }
}

impl<T> From<T> for EzspNetworkManager<T>
where
    T: Ezsp + Send,
{
    fn from(ezsp: T) -> Self {
        Self::new(ezsp)
    }
}
