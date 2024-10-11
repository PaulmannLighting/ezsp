use crate::{frame, Resolve};
use le_stream::derive::FromLeStream;
use le_stream::ToLeStream;
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

impl ToLeStream for Command {
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

    fn to_le_stream(self) -> Self::Iter {
        self.endpoint
            .to_le_stream()
            .into_iter()
            .chain(self.profile_id.to_le_stream())
            .chain(self.device_id.to_le_stream())
            .chain(self.app_flags.to_le_stream())
            .chain((self.input_clusters.len() as u8).to_le_stream())
            .chain(
                self.input_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
            .chain((self.output_clusters.len() as u8).to_le_stream())
            .chain(
                self.output_clusters
                    .into_iter()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
