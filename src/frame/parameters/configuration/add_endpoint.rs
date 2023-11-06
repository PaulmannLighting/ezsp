use crate::ezsp::Status;
use crate::read_write::Readable;
use crate::types::ByteSizedVec;
use rw_exact_ext::ReadExactExt;
use std::io;
use std::io::Read;
use std::iter::{once, Once};
use std::vec::IntoIter;

pub const ID: u16 = 0x0002;

/// Configures endpoint information on the NCP.
///
/// The NCP does not remember these settings after a reset.
/// Endpoints can be added by the Host after the NCP has reset.
/// Once the status of the stack changes to [`crate::ember::Status::NetworkUp`],
/// endpoints can no longer be added and this command will respond with
/// [`Status::Error`]`(`[`Error::InvalidCall`][`crate::ezsp::error::Error::InvalidCall`]`)`.
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
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
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

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn input_cluster_count(&self) -> u8 {
        self.input_clusters.len() as u8
    }

    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn output_cluster_count(&self) -> u8 {
        self.output_clusters.len() as u8
    }

    #[must_use]
    pub fn input_cluster_list(&self) -> &[u16] {
        &self.input_clusters
    }

    #[must_use]
    pub fn output_cluster_list(&self) -> &[u16] {
        &self.output_clusters
    }

    fn read_clusters<R>(src: &mut R, count: usize) -> io::Result<ByteSizedVec<u16>>
    where
        R: Read,
    {
        Ok(src
            .read_vec_exact(2 * count)?
            .chunks_exact(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some(u16::from_le_bytes([chunk[0], chunk[1]]))
                } else {
                    None
                }
            })
            .collect())
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut parameters =
            Vec::with_capacity(8 + self.input_clusters.len() * 2 + self.output_clusters.len() * 2);
        parameters.push(self.endpoint);
        parameters.extend_from_slice(&self.profile_id.to_le_bytes());
        parameters.extend_from_slice(&self.device_id.to_le_bytes());
        parameters.push(self.app_flags);
        parameters.push(self.input_cluster_count());
        parameters.push(self.output_cluster_count());
        self.input_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_le_bytes()));
        self.output_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_le_bytes()));
        parameters.into_iter()
    }
}

impl Readable for Command {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let endpoint = src.read_num_le()?;
        let profile_id = src.read_num_le()?;
        let device_id = src.read_num_le()?;
        let [app_flags, input_cluster_count, output_cluster_count] = src.read_array_exact()?;
        Ok(Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters: Self::read_clusters(src, input_cluster_count as usize)?,
            output_clusters: Self::read_clusters(src, output_cluster_count as usize)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.status.into())
    }
}

impl Readable for Response {
    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let status: u8 = src.read_num_le()?;
        Ok(Self {
            status: status.try_into()?,
        })
    }
}
