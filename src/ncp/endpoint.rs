use log::debug;

use crate::types::ByteSizedVec;
use crate::{Configuration, Error};

/// An application endpoint registered on the NCP during startup.
///
/// The descriptor records the endpoint and device identifiers, application
/// flags, and input/output clusters. [`Ncp`](crate::Ncp) retains the output
/// clusters to select a source endpoint for outgoing APS messages.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Endpoint {
    pub(crate) id: u8,
    pub(crate) profile_id: u16,
    pub(crate) device_id: u16,
    pub(crate) app_flags: u8,
    pub(crate) input_clusters: ByteSizedVec<u16>,
    pub(crate) output_clusters: ByteSizedVec<u16>,
}

impl Endpoint {
    pub(crate) async fn add_to<T>(self, target: &mut T) -> Result<(), Error>
    where
        T: Configuration,
    {
        debug!(
            "Adding endpoint: {:#04X}, profile: {:#06X}, device_id: {:#06X}, app_flags: {:#04X}, input clusters: {:X?}, output clusters: {:X?}",
            self.id,
            self.profile_id,
            self.device_id,
            self.app_flags,
            self.input_clusters,
            self.output_clusters,
        );

        target
            .add_endpoint(
                self.id,
                self.profile_id,
                self.device_id,
                self.app_flags,
                self.input_clusters,
                self.output_clusters,
            )
            .await
    }
}
