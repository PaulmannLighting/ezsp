use crate::frame::Parameters;
use crate::status::Status;
use std::io;
use std::io::Read;
use std::num::TryFromIntError;
use std::sync::Arc;

pub const ID: u16 = 0x0002;

/// Configures endpoint information on the NCP.
///
/// The NCP does not remember these settings after a reset.
/// Endpoints can be added by the Host after the NCP has reset.
/// Once the status of the stack changes to EMBER_NETWORK_UP,
/// endpoints can no longer be added and this command
/// will respond with [`Status::Error`]`(`[`Error::InvalidCall`][crate::status::Error::InvalidCall]`)`.
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
    /// Creates a new [`Command`]
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

impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        let mut parameters = Vec::with_capacity(
            8 + command.input_clusters.len() * 2 + command.output_clusters.len() * 2,
        );
        parameters.push(command.endpoint);
        parameters.extend_from_slice(&command.profile_id.to_be_bytes());
        parameters.extend_from_slice(&command.device_id.to_be_bytes());
        parameters.push(command.app_flags);
        parameters.push(command.input_cluster_count);
        parameters.push(command.output_cluster_count);
        command
            .input_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_be_bytes()));
        command
            .output_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_be_bytes()));
        parameters
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

impl From<Response> for Vec<u8> {
    fn from(response: Response) -> Self {
        vec![response.status.into()]
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
