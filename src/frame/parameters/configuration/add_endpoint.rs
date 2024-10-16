use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::ToLeStream;
use std::array::IntoIter;
use std::iter::{Chain, FlatMap};

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
    pub const fn new(
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
            clusters: Clusters {
                input_clusters: input_cluster_list,
                output_clusters: output_cluster_list,
            },
        }
    }
}

/// Helper struct to deal with special serialization of the cluster lists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clusters {
    input_clusters: ByteSizedVec<u16>,
    output_clusters: ByteSizedVec<u16>,
}

/// Manual implementation of `ToLeStream` because the length hints of both `input_clusters`
/// and `output_clusters` must be output before their actual elements:
///
/// I.e. the order of output bytes is:
///
/// 1. Number of input clusters
/// 2. Number of output clusters
/// 3. Input cluster list
/// 4. Output cluster list
///
/// See p. 50 of the `EZSP` specification.
impl ToLeStream for Clusters {
    type Iter = Chain<
        Chain<
            Chain<IntoIter<u8, 1>, IntoIter<u8, 1>>,
            FlatMap<<ByteSizedVec<u16> as IntoIterator>::IntoIter, [u8; 2], fn(u16) -> [u8; 2]>,
        >,
        FlatMap<<ByteSizedVec<u16> as IntoIterator>::IntoIter, [u8; 2], fn(u16) -> [u8; 2]>,
    >;

    #[allow(trivial_casts, clippy::cast_possible_truncation)]
    fn to_le_stream(self) -> Self::Iter {
        (self.input_clusters.len() as u8)
            .to_le_stream()
            .chain((self.output_clusters.len() as u8).to_le_stream())
            .chain(
                self.input_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as _),
            )
            .chain(
                self.output_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as _),
            )
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
