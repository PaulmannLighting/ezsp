use crate::ezsp::Status;
use crate::frame::Parameters;
use std::array::IntoIter;
use std::io::Read;
use std::iter::{once, Chain, Copied, FlatMap, Once};
use std::num::TryFromIntError;
use std::slice::Iter;
use std::sync::Arc;
use std::{io, vec};

pub const ID: u16 = 0x0002;

/// Configures endpoint information on the NCP.
///
/// The NCP does not remember these settings after a reset.
/// Endpoints can be added by the Host after the NCP has reset.
/// Once the status of the stack changes to EMBER_NETWORK_UP,
/// endpoints can no longer be added and this command
/// will respond with [`Status::Error`]`(`[`Error::InvalidCall`][crate::ezsp::Error::InvalidCall]`)`.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    input_cluster_count: u8,
    output_cluster_count: u8,
    input_clusters: Arc<[u16]>,
    output_clusters: Arc<[u16]>,
}

impl Command {
    /// Creates new [`Command`] payload
    ///
    /// # Errors
    /// Returns a [`TryFromIntError`] if the size of either `input_clusters`
    /// or `output_clusters` exceeds the bounds of an u8.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: Arc<[u16]>,
        output_clusters: Arc<[u16]>,
    ) -> Result<Self, TryFromIntError> {
        Ok(Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_cluster_count: input_clusters.len().try_into()?,
            output_cluster_count: output_clusters.len().try_into()?,
            input_clusters,
            output_clusters,
        })
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
        self.input_cluster_count
    }

    #[must_use]
    pub const fn output_cluster_count(&self) -> u8 {
        self.output_cluster_count
    }

    #[must_use]
    pub fn input_cluster_list(&self) -> &[u16] {
        &self.input_clusters
    }

    #[must_use]
    pub fn output_cluster_list(&self) -> &[u16] {
        &self.output_clusters
    }

    fn read_clusters<R>(reader: &mut R, count: usize) -> io::Result<Vec<u16>>
    where
        R: Read,
    {
        let mut clusters = vec![0; 2 * count];
        reader.read_exact(&mut clusters)?;
        Ok(clusters
            .chunks_exact(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some(u16::from_be_bytes([chunk[0], chunk[1]]))
                } else {
                    None
                }
            })
            .collect())
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = Chain<
        Chain<
            Chain<Chain<Chain<Once<u8>, IntoIter<u8, 2>>, IntoIter<u8, 2>>, IntoIter<u8, 3>>,
            FlatMap<Copied<Iter<'static, u16>>, [u8; 2], fn(u16) -> [u8; 2]>,
        >,
        FlatMap<Copied<Iter<'static, u16>>, [u8; 2], fn(u16) -> [u8; 2]>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        once(self.endpoint)
            .chain(self.profile_id.to_be_bytes())
            .chain(self.device_id.to_be_bytes())
            .chain([
                self.app_flags,
                self.input_cluster_count,
                self.output_cluster_count,
            ])
            .chain(
                self.input_clusters
                    .into_iter()
                    .copied()
                    .flat_map(u16::to_be_bytes as fn(u16) -> [u8; 2]),
            )
            .chain(
                self.output_clusters
                    .into_iter()
                    .copied()
                    .flat_map(u16::to_be_bytes as fn(u16) -> [u8; 2]),
            )
    }
}

impl Parameters<u16> for Command {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @
        [endpoint, profile_id_low, profile_id_high, device_id_low, device_id_high, app_flags, input_cluster_count, output_cluster_count] =
            [0; 8];
        src.read_exact(&mut buffer)?;
        let input_clusters = Self::read_clusters(src, input_cluster_count.into())?;
        let output_clusters = Self::read_clusters(src, output_cluster_count.into())?;
        Ok(Self {
            endpoint,
            profile_id: u16::from_be_bytes([profile_id_low, profile_id_high]),
            device_id: u16::from_be_bytes([device_id_low, device_id_high]),
            app_flags,
            input_cluster_count,
            output_cluster_count,
            input_clusters: input_clusters.into(),
            output_clusters: output_clusters.into(),
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

impl Parameters<u16> for Response {
    const FRAME_ID: u16 = ID;

    fn read_from<R>(src: &mut R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut buffer @ [status] = [0; 1];
        src.read_exact(&mut buffer)?;
        Ok(Self {
            status: Status::try_from(status)?,
        })
    }
}
