use crate::ember::PanId;
use crate::ezsp::decision;
use crate::frame::parameters::{read_attribute, version};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;

pub trait Configuration: Transport {
    /// The command allows the Host to specify the desired EZSP version and must be sent
    /// before any other command.
    /// The response provides information about the firmware running on the NCP.
    fn version(
        &self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;

    /// Legacy implementation of [`version()`](Self::version) using a shortened header.
    fn legacy_version(
        &self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;

    /// Reads a configuration value from the NCP.
    fn get_configuration_value(
        &self,
        config_id: u8,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Writes a configuration value to the NCP. Configuration values can be modified by the Host after the NCP has reset.
    /// Once the status of the stack changes to [`Status::NetworkUp`](crate::ember::Status::NetworkUp),
    /// configuration values can no longer be modified
    /// and this command will respond with [`Error::InvalidCall`](crate::ezsp::Error::InvalidCall).
    fn set_configuration_value(
        &self,
        config_id: u8,
        value: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Read attribute data on NCP endpoints.
    fn read_attribute(
        &self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> impl Future<Output = Result<read_attribute::Response, Error>> + Send;

    /// Write attribute data on NCP endpoints.
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

    /// Configures endpoint information on the NCP.
    /// The NCP does not remember these settings after a reset.
    /// Endpoints can be added by the Host after the NCP has reset.
    /// Once the status of the stack changes to [`Status::NetworkUp`](crate::ember::Status::NetworkUp),
    /// endpoints can no longer be added and this command will respond with [`Error::InvalidCall`](crate::ezsp::Error::InvalidCall).
    fn add_endpoint(
        &self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows the Host to change the policies used by the NCP to make fast decisions.
    fn set_policy(&self, policy_id: u8, decision_id: u8)
        -> impl Future<Output = Result<(), Error>>;

    /// Allows the Host to read the policies used by the NCP to make fast decisions.
    fn get_policy(&self, policy_id: u8)
        -> impl Future<Output = Result<decision::Id, Error>> + Send;

    /// Triggers a pan id update message.
    fn send_pan_id_update(
        &self,
        new_pan: PanId,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Reads a value from the NCP.
    fn get_value(
        &self,
        value_id: u8,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Reads a value from the NCP but passes an extra argument specific to the value being retrieved.
    fn get_extended_value(
        &self,
        value_id: u8,
        characteristics: u32,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Writes a value to the NCP.
    fn set_value(
        &self,
        value_id: u8,
        value: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
    fn set_passive_ack_config(
        &self,
        config: u8,
        min_acks_needed: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
