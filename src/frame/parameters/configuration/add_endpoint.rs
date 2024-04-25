use crate::error::Resolve;
use le_stream::derive::FromLeBytes;
use le_stream::ToLeBytes;
use std::array::IntoIter;
use std::iter::{Chain, FlatMap};

use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0002;
const SIZE: usize = 1 + 2 + 2 + 1 + 2 * (2 * u8::MAX as usize);

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    input_clusters: ByteSizedVec<u16>,
    output_clusters: ByteSizedVec<u16>,
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
            input_clusters: input_cluster_list,
            output_clusters: output_cluster_list,
        }
    }
}

impl ToLeBytes for Command {
    type Iter = Chain<
        Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<Chain<IntoIter<u8, 1>, IntoIter<u8, 2>>, IntoIter<u8, 2>>,
                        IntoIter<u8, 1>,
                    >,
                    IntoIter<u8, 1>,
                >,
                FlatMap<<ByteSizedVec<u16> as IntoIterator>::IntoIter, [u8; 2], fn(u16) -> [u8; 2]>,
            >,
            IntoIter<u8, 1>,
        >,
        FlatMap<<ByteSizedVec<u16> as IntoIterator>::IntoIter, [u8; 2], fn(u16) -> [u8; 2]>,
    >;

    fn to_le_bytes(self) -> Self::Iter {
        self.endpoint
            .to_le_bytes()
            .into_iter()
            .chain(self.profile_id.to_le_bytes())
            .chain(self.device_id.to_le_bytes())
            .chain(self.app_flags.to_le_bytes())
            .chain((self.input_clusters.len() as u8).to_le_bytes())
            .chain(
                self.input_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
            .chain((self.output_clusters.len() as u8).to_le_bytes())
            .chain(
                self.output_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
