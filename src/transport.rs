mod ashv2;

use crate::types::ByteSizedVec;
use crate::{ember, ezsp, Error};
use le_stream::ToLeBytes;
use std::future::Future;

pub trait Transport {
    fn next_command<T>(&mut self, frame_id: u16, parameters: T) -> Vec<u8>
    where
        T: ToLeBytes;
}

pub trait Ezsp: Transport {
    fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<ezsp::Status, Error>>;

    fn add_or_update_key_table_entry(
        &mut self,
        address: ember::Eui64,
        link_key: bool,
        key_data: ember::key::Data,
    ) -> impl Future<Output = Result<ember::Status, Error>>;
}
