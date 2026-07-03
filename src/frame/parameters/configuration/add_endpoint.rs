//! Parameters for the [`Configuration::add_endpoint`](crate::Configuration::add_endpoint) command.

use std::iter::{Chain, FlatMap};

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::types::ByteSizedVec;

crate::frame::parameters::frame!(
    0x0002,
    { endpoint: u8, profile_id: u16, device_id: u16, app_flags: u8, clusters: Clusters },
    impl {
        impl Command {
            /// Creates command parameters.
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
    },
    { status: u8 },
    impl {
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
    }
);

/// Helper struct to deal with special serialization of the cluster lists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clusters {
    input_cluster_counts: u8,
    output_cluster_counts: u8,
    #[expect(clippy::struct_field_names)]
    input_clusters: ByteSizedVec<u16>,
    #[expect(clippy::struct_field_names)]
    output_clusters: ByteSizedVec<u16>,
}

impl Clusters {
    /// Creates command parameters.
    #[must_use]
    pub fn new(input_clusters: ByteSizedVec<u16>, output_clusters: ByteSizedVec<u16>) -> Self {
        Self {
            #[expect(clippy::cast_possible_truncation)]
            input_cluster_counts: input_clusters.len() as u8,
            #[expect(clippy::cast_possible_truncation)]
            output_cluster_counts: output_clusters.len() as u8,
            input_clusters,
            output_clusters,
        }
    }
}

impl FromLeStream for Clusters {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let input_cluster_counts = u8::from_le_stream(bytes.by_ref())?;
        let output_cluster_counts = u8::from_le_stream(bytes.by_ref())?;
        let mut input_clusters = ByteSizedVec::new();
        let mut output_clusters = ByteSizedVec::new();

        for _ in 0..input_cluster_counts {
            input_clusters
                .push(u16::from_le_stream(bytes.by_ref())?)
                .ok()?;
        }

        for _ in 0..output_cluster_counts {
            output_clusters
                .push(u16::from_le_stream(bytes.by_ref())?)
                .ok()?;
        }

        Some(Self {
            input_cluster_counts,
            output_cluster_counts,
            input_clusters,
            output_clusters,
        })
    }
}

impl ToLeStream for Clusters {
    type Iter = Chain<
        Chain<
            Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
            FlatMap<
                <ByteSizedVec<u16> as IntoIterator>::IntoIter,
                <u16 as ToLeStream>::Iter,
                fn(u16) -> <u16 as ToLeStream>::Iter,
            >,
        >,
        FlatMap<
            <ByteSizedVec<u16> as IntoIterator>::IntoIter,
            <u16 as ToLeStream>::Iter,
            fn(u16) -> <u16 as ToLeStream>::Iter,
        >,
    >;

    fn to_le_stream(self) -> Self::Iter {
        #[expect(trivial_casts)]
        self.input_cluster_counts
            .to_le_stream()
            .chain(self.output_cluster_counts.to_le_stream())
            .chain(
                self.input_clusters
                    .into_iter()
                    .flat_map(ToLeStream::to_le_stream as _),
            )
            .chain(
                self.output_clusters
                    .into_iter()
                    .flat_map(ToLeStream::to_le_stream as _),
            )
    }
}
