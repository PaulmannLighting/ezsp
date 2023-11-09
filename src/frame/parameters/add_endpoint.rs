use crate::types::{ByteSizedVec, EzspStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{Error, FromLeBytes, ToLeBytes};
use std::array::IntoIter;
use std::iter::{Chain, Copied, FlatMap};
use std::slice::Iter;

pub const ID: u16 = 0x0002;

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

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    #[must_use]
    pub const fn app_flags(&self) -> u8 {
        self.app_flags
    }

    #[must_use]
    pub const fn input_cluster_count(&self) -> u8 {
        self.input_clusters.len() as u8
    }

    #[must_use]
    pub const fn output_cluster_count(&self) -> u8 {
        self.output_clusters.len() as u8
    }

    #[must_use]
    pub const fn input_cluster_list(&self) -> &ByteSizedVec<u16> {
        &self.input_clusters
    }

    #[must_use]
    pub const fn output_cluster_list(&self) -> &ByteSizedVec<u16> {
        &self.output_clusters
    }
}

impl FromLeBytes for Command {
    fn from_le_bytes<T>(bytes: &mut T) -> le_stream::Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        let endpoint = <u8 as FromLeBytes>::from_le_bytes(bytes)?;
        let profile_id = <u16 as FromLeBytes>::from_le_bytes(bytes)?;
        let device_id = <u16 as FromLeBytes>::from_le_bytes(bytes)?;
        let app_flags = <u8 as FromLeBytes>::from_le_bytes(bytes)?;
        let input_cluster_count = <u8 as FromLeBytes>::from_le_bytes(bytes)?;
        let output_cluster_count = <u8 as FromLeBytes>::from_le_bytes(bytes)?;
        let mut input_clusters = ByteSizedVec::<u16>::new();
        let mut output_clusters = ByteSizedVec::<u16>::new();

        let mut buffer = [0; 2];

        for _ in 0..input_cluster_count {
            for byte in &mut buffer {
                *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
            }

            input_clusters
                .push(u16::from_le_bytes(buffer))
                .expect("buffer overflow");
        }

        for _ in 0..output_cluster_count {
            for byte in &mut buffer {
                *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
            }

            output_clusters
                .push(u16::from_le_bytes(buffer))
                .expect("buffer overflow");
        }

        Ok(Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        })
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
                IntoIter<u8, 1>,
            >,
            FlatMap<Copied<Iter<'static, u16>>, [u8; 2], fn(u16) -> [u8; 2]>,
        >,
        FlatMap<Copied<Iter<'static, u16>>, [u8; 2], fn(u16) -> [u8; 2]>,
    >;

    fn to_le_bytes(&self) -> Self::Iter {
        self.endpoint
            .to_le_bytes()
            .into_iter()
            .chain(self.profile_id.to_le_bytes())
            .chain(self.device_id.to_le_bytes())
            .chain(self.app_flags.to_le_bytes())
            .chain(self.input_cluster_count().to_le_bytes())
            .chain(self.output_cluster_count().to_le_bytes())
            .chain(
                self.input_clusters
                    .iter()
                    .copied()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
            .chain(
                self.output_clusters
                    .iter()
                    .copied()
                    .flat_map(u16::to_le_bytes as fn(u16) -> [u8; 2]),
            )
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EzspStatus {
        self.status
    }
}
