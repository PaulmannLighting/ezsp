use crate::frame::parameters::wwah::{
    get_parent_classification_enabled, set_parent_classification_enabled,
};
use crate::{Error, Transport};
use std::future::Future;

/// The `Wwah` trait provides an interface for the Work With All Hubs (WWAH) protocol.
pub trait Wwah {
    /// Gets whether to use parent classification when processing beacons during a join or rejoin.
    ///
    /// Parent classification considers whether a received beacon indicates trust center
    /// connectivity and long uptime on the network.
    fn get_parent_classification_enabled(&mut self) -> impl Future<Output = Result<bool, Error>>;

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

    async fn set_parent_classification_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        self.communicate::<_, set_parent_classification_enabled::Response>(
            set_parent_classification_enabled::Command::new(enabled),
        )
        .await
        .map(drop)
    }
}
