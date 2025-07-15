use std::future::Future;

use crate::error::Error;
use crate::frame::parameters::wwah::{
    get_parent_classification_enabled, is_hub_connected, is_uptime_long, set_hub_connectivity,
    set_long_uptime, set_parent_classification_enabled,
};
use crate::transport::Transport;

/// The `Wwah` trait provides an interface for the Work With All Hubs (WWAH) protocol.
pub trait Wwah {
    /// Gets whether to use parent classification when processing beacons during a join or rejoin.
    ///
    /// Parent classification considers whether a received beacon indicates trust center
    /// connectivity and long uptime on the network.
    fn get_parent_classification_enabled(&mut self) -> impl Future<Output = Result<bool, Error>>;

    /// Checks if the hub is connected or not.
    #[allow(clippy::wrong_self_convention)]
    fn is_hub_connected(&mut self) -> impl Future<Output = Result<bool, Error>>;

    /// Checks if the device uptime is long or short.
    #[allow(clippy::wrong_self_convention)]
    fn is_uptime_long(&mut self) -> impl Future<Output = Result<bool, Error>>;

    /// Sets the hub connectivity to be `true` or `false`.
    fn set_hub_connectivity(&mut self, connected: bool) -> impl Future<Output = Result<(), Error>>;

    /// Sets the device uptime to be long or short.
    fn set_long_uptime(&mut self, has_long_uptime: bool)
    -> impl Future<Output = Result<(), Error>>;

    /// Sets whether to use parent classification when processing beacons during a join or rejoin.
    ///
    /// Parent classification considers whether a received beacon indicates trust center
    /// connectivity and long uptime on the network.
    fn set_parent_classification_enabled(
        &mut self,
        enabled: bool,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl<T> Wwah for T
where
    T: Transport,
{
    async fn get_parent_classification_enabled(&mut self) -> Result<bool, Error> {
        self.communicate::<_, get_parent_classification_enabled::Response>(
            get_parent_classification_enabled::Command,
        )
        .await
        .map(Into::into)
    }

    async fn is_hub_connected(&mut self) -> Result<bool, Error> {
        self.communicate::<_, is_hub_connected::Response>(is_hub_connected::Command)
            .await
            .map(Into::into)
    }

    async fn is_uptime_long(&mut self) -> Result<bool, Error> {
        self.communicate::<_, is_uptime_long::Response>(is_uptime_long::Command)
            .await
            .map(Into::into)
    }

    async fn set_hub_connectivity(&mut self, connected: bool) -> Result<(), Error> {
        self.communicate::<_, set_hub_connectivity::Response>(set_hub_connectivity::Command::new(
            connected,
        ))
        .await
        .map(drop)
    }

    async fn set_long_uptime(&mut self, has_long_uptime: bool) -> Result<(), Error> {
        self.communicate::<_, set_long_uptime::Response>(set_long_uptime::Command::new(
            has_long_uptime,
        ))
        .await
        .map(drop)
    }

    async fn set_parent_classification_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        self.communicate::<_, set_parent_classification_enabled::Response>(
            set_parent_classification_enabled::Command::new(enabled),
        )
        .await
        .map(drop)
    }
}
