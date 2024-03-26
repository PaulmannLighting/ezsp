use crate::ember::PanId;
use crate::ezsp::decision;
use crate::frame::parameters::{read_attribute, version};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;

pub trait Configuration: Transport {
    fn version(
        &self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;

    fn legacy_version(
        &self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;

    fn get_configuration_value(
        &self,
        config_id: u8,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    fn set_configuration_value(
        &self,
        config_id: u8,
        value: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn read_attribute(
        &self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> impl Future<Output = Result<read_attribute::Response, Error>> + Send;

    #[allow(clippy::too_many_arguments)]
    fn write_attribute(
        &self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
        just_test: bool,
        data_type: u8,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn add_endpoint(
        &self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn set_policy(&self, policy_id: u8, decision_id: u8)
        -> impl Future<Output = Result<(), Error>>;
    fn get_policy(&self, policy_id: u8)
        -> impl Future<Output = Result<decision::Id, Error>> + Send;

    fn send_pan_id_update(
        &self,
        new_pan: PanId,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    fn get_value(
        &self,
        value_id: u8,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    fn get_extended_value(
        &self,
        value_id: u8,
        characteristics: u32,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    fn set_value(
        &self,
        value_id: u8,
        value: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn set_passive_ack_config(
        &self,
        config: u8,
        min_acks_needed: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
