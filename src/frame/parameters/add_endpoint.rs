use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0002;
const SIZE: usize = 1 + 2 + 2 + 1 + 2 * (2 * u8::MAX as usize);

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
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

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
