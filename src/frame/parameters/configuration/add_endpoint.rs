//! Parameters for the [`Configuration::add_endpoint`](crate::Configuration::add_endpoint) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0002;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    clusters: Clusters,
}

impl Command {
    #[must_use]
    pub fn new(
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_cluster_list: ByteSizedVec<u16>,
        output_cluster_list: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            clusters: Clusters::new(input_cluster_list, output_cluster_list),
        }
    }
}

/// Helper struct to deal with special serialization of the cluster lists.
#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Clusters {
    input_cluster_counts: u8,
    output_cluster_counts: u8,
    #[allow(clippy::struct_field_names)]
    input_clusters: ByteSizedVec<u16>,
    #[allow(clippy::struct_field_names)]
    output_clusters: ByteSizedVec<u16>,
}

impl Clusters {
    #[must_use]
    pub fn new(input_clusters: ByteSizedVec<u16>, output_clusters: ByteSizedVec<u16>) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            input_cluster_counts: input_clusters.len() as u8,
            #[allow(clippy::cast_possible_truncation)]
            output_cluster_counts: output_clusters.len() as u8,
            input_clusters,
            output_clusters,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
