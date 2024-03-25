use crate::ember::PanId;
use crate::ezsp::decision;
use crate::frame::parameters::{read_attribute, version};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;

pub trait Configuration: Transport {
    fn version(
        &mut self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response<u16>, Error>>;
    fn legacy_version(
        &mut self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response<u8>, Error>>;
    fn get_configuration_value(
        &mut self,
        config_id: u8,
    ) -> impl Future<Output = Result<u16, Error>>;
    fn set_configuration_value(
        &mut self,
        config_id: u8,
        value: u16,
    ) -> impl Future<Output = Result<(), Error>>;
    fn read_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> impl Future<Output = Result<read_attribute::Response, Error>>;
    fn write_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
        just_test: bool,
        data_type: u8,
        data: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>>;
    fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<(), Error>>;
    fn set_policy(
        &mut self,
        policy_id: u8,
        decision_id: u8,
    ) -> impl Future<Output = Result<(), Error>>;
    fn get_policy(&mut self, policy_id: u8) -> impl Future<Output = Result<decision::Id, Error>>;
    fn send_pan_id_update(&mut self, new_pan: PanId) -> impl Future<Output = Result<bool, Error>>;
    fn get_value(&mut self, value_id: u8) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>>;
    fn get_extended_value(
        &mut self,
        value_id: u8,
        characteristics: u32,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>>;
    fn set_value(
        &mut self,
        value_id: u8,
        value: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>>;
    fn set_passive_ack_config(
        &mut self,
        config: u8,
        min_acks_needed: u8,
    ) -> impl Future<Output = Result<(), Error>>;
}
