mod add_endpoint;
mod get_configuration_value;
mod get_extended_value;
mod get_policy;
mod get_value;
mod read_attribute;
mod send_pan_id_update;
mod set_configuration_value;
mod set_passive_ack_config;
mod set_policy;
mod set_value;
mod version;
mod write_attribute;

use crate::transaction::Transaction;
use crate::types::ByteSizedVec;

pub trait Configuration {
    fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Transaction<add_endpoint::Command, add_endpoint::Response> {
        Transaction::new(add_endpoint::Command::new(
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        ))
    }
}
