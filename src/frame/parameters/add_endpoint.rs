use crate::dongle::{BootloaderHandler, Dongle, ZigBeeTransportReceive};
use crate::ezsp::Status;
use crate::frame::parameters::add_endpoint;
use crate::types::ByteSizedVec;
use crate::{ezsp, Protocol};
use le_stream::derive::{FromLeBytes, ToLeBytes};
use le_stream::{Error, FromLeBytes, ToLeBytes};
use serialport::SerialPort;

pub const ID: u16 = 0x0002;
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
                .expect("Input clusters buffer should have sufficient capacity.");
        }

        for _ in 0..output_cluster_count {
            for byte in &mut buffer {
                *byte = bytes.next().ok_or(Error::UnexpectedEndOfStream)?;
            }

            output_clusters
                .push(u16::from_le_bytes(buffer))
                .expect("Output clusters buffer should have sufficient capacity.");
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
    type Iter = <heapless::Vec<u8, SIZE> as IntoIterator>::IntoIter;

    fn to_le_bytes(self) -> Self::Iter {
        self.endpoint
            .to_le_bytes()
            .into_iter()
            .chain(self.profile_id.to_le_bytes())
            .chain(self.device_id.to_le_bytes())
            .chain(self.app_flags.to_le_bytes())
            .chain(
                u8::try_from(self.input_clusters.len())
                    .expect("Input clusters buffer size should never exceed u8.")
                    .to_le_bytes(),
            )
            .chain(
                u8::try_from(self.output_clusters.len())
                    .expect("Output clusters buffer size should never exceed u8.")
                    .to_le_bytes(),
            )
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
            .collect::<heapless::Vec<u8, SIZE>>()
            .into_iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl<B, T, P, S> Dongle<B, T, P, S>
where
    B: BootloaderHandler,
    T: ZigBeeTransportReceive,
    P: Protocol,
    S: SerialPort,
{
    /// Add and endpoint.
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors.
    pub fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> std::io::Result<Status> {
        let command = Command::new(
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        );
        for byte in command.to_le_bytes() {
            self.write(byte)?;
        }

        todo!("Receive response")
    }
}
