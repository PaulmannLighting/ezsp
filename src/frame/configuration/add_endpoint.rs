use crate::frame::header::Header;
use crate::frame::Frame;
use crate::status::Status;
use num_traits::ToPrimitive;
use std::sync::Arc;

const ID: u16 = 0x0002;

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    header: Header,
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    input_clusters: Arc<[u16]>,
    output_clusters: Arc<[u16]>,
}

impl Command {
    pub const fn new(
        header: Header,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: Arc<[u16]>,
        output_clusters: Arc<[u16]>,
    ) -> Self {
        Self {
            header,
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        }
    }

    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    pub const fn app_flags(&self) -> u8 {
        self.app_flags
    }

    pub const fn input_cluster_count(&self) -> u8 {
        self.input_clusters
            .len()
            .try_into()
            .expect("input cluster length exceeds u8")
    }

    pub const fn output_cluster_count(&self) -> u8 {
        self.output_clusters
            .len()
            .try_into()
            .expect("output cluster length exceeds u8")
    }

    pub const fn input_cluster_list(&self) -> &[u16] {
        &self.input_clusters
    }

    pub const fn output_cluster_list(&self) -> &[u16] {
        &self.output_clusters
    }
}

impl Frame<ID> for Command {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        let mut parameters =
            Vec::with_capacity(8 + self.input_clusters.len() * 2 + self.output_clusters.len() * 2);
        parameters.push(self.endpoint);
        parameters.extend_from_slice(&self.profile_id.to_be_bytes());
        parameters.extend_from_slice(&self.device_id.to_be_bytes());
        parameters.push(self.app_flags);
        parameters.push(self.input_cluster_count());
        parameters.push(self.output_cluster_count());
        self.input_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_be_bytes()));
        self.output_clusters
            .iter()
            .for_each(|cluster| parameters.extend_from_slice(&cluster.to_be_bytes()));
        parameters
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    header: Header,
    status: Status,
}

impl Response {
    pub const fn new(header: Header, status: Status) -> Self {
        Self { header, status }
    }

    pub const fn status(&self) -> &Status {
        &self.status
    }
}

impl Frame<ID> for Response {
    fn header(&self) -> &Header {
        &self.header
    }

    fn parameters(&self) -> Vec<u8> {
        vec![self.status.to_u8().expect("could not convert status to u8")]
    }
}
