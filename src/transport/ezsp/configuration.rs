use std::future::Future;

use crate::ember::PanId;
use crate::error::Error;
use crate::ezsp::config::Id;
use crate::ezsp::value::ExtendedId;
use crate::ezsp::{decision, policy, value};
use crate::frame::parameters::configuration::{
    add_endpoint, get_configuration_value, get_extended_value, get_policy, get_value,
    read_attribute, send_pan_id_update, set_configuration_value, set_passive_ack_config,
    set_policy, set_value, version, write_attribute,
};
use crate::parameters::configuration::write_attribute::Attribute;
use crate::transport::Transport;
use crate::types::ByteSizedVec;

/// The `Configuration` trait provides an interface for the configuration commands.
pub trait Configuration {
    /// Configures endpoint information on the NCP.
    /// The NCP does not remember these settings after a reset.
    /// Endpoints can be added by the Host after the NCP has reset.
    /// Once the status of the stack changes to [`Status::NetworkUp`](crate::ember::Status::NetworkUp),
    /// endpoints can no longer be added and this command will respond with [`Error::InvalidCall`](crate::ezsp::Error::InvalidCall).
    fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Reads a configuration value from the NCP.
    fn get_configuration_value(
        &mut self,
        config_id: Id,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Reads a value from the NCP but passes an extra argument specific to the value being retrieved.
    fn get_extended_value(
        &mut self,
        value_id: ExtendedId,
        characteristics: u32,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Allows the Host to read the policies used by the NCP to make fast decisions.
    fn get_policy(
        &mut self,
        policy_id: policy::Id,
    ) -> impl Future<Output = Result<decision::Id, Error>> + Send;

    /// Reads a value from the NCP.
    fn get_value(
        &mut self,
        value_id: value::Id,
    ) -> impl Future<Output = Result<ByteSizedVec<u8>, Error>> + Send;

    /// Read attribute data on NCP endpoints.
    fn read_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> impl Future<Output = Result<read_attribute::Attribbute, Error>> + Send;

    /// Triggers a pan id update message.
    fn send_pan_id_update(
        &mut self,
        new_pan: PanId,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Writes a configuration value to the NCP. Configuration values can be modified by the Host after the NCP has reset.
    /// Once the status of the stack changes to [`Status::NetworkUp`](crate::ember::Status::NetworkUp),
    /// configuration values can no longer be modified
    /// and this command will respond with [`Error::InvalidCall`](crate::ezsp::Error::InvalidCall).
    fn set_configuration_value(
        &mut self,
        config_id: Id,
        value: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows the Host to control the broadcast behaviour of a routing device used by the NCP.
    fn set_passive_ack_config(
        &mut self,
        config: u8,
        min_acks_needed: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Allows the Host to change the policies used by the NCP to make fast decisions.
    fn set_policy(
        &mut self,
        policy_id: policy::Id,
        decision_id: decision::Id,
    ) -> impl Future<Output = Result<(), Error>>;

    /// Writes a value to the NCP.
    fn set_value(
        &mut self,
        value_id: value::Id,
        value: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// The command allows the Host to specify the desired EZSP version and must be sent
    /// before any other command.
    /// The response provides information about the firmware running on the NCP.
    fn version(
        &mut self,
        desired_protocol_version: u8,
    ) -> impl Future<Output = Result<version::Response, Error>> + Send;

    /// Write attribute data on NCP endpoints.
    #[allow(clippy::too_many_arguments)]
    fn write_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute: &Attribute,
        just_test: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Configuration for T
where
    T: Transport,
{
    async fn add_endpoint(
        &mut self,
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_clusters: ByteSizedVec<u16>,
        output_clusters: ByteSizedVec<u16>,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, add_endpoint::Response>(add_endpoint::Command::new(
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_clusters,
            output_clusters,
        ))
        .await?
        .try_into()
    }

    async fn get_configuration_value(&mut self, config_id: Id) -> Result<u16, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_configuration_value::Response>(
            get_configuration_value::Command::new(config_id),
        )
        .await?
        .try_into()
    }

    async fn get_extended_value(
        &mut self,
        value_id: ExtendedId,
        characteristics: u32,
    ) -> Result<ByteSizedVec<u8>, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_extended_value::Response>(get_extended_value::Command::new(
            value_id,
            characteristics,
        ))
        .await?
        .try_into()
    }

    async fn get_policy(&mut self, policy_id: policy::Id) -> Result<decision::Id, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_policy::Response>(get_policy::Command::new(policy_id))
            .await?
            .try_into()
    }

    async fn get_value(&mut self, value_id: value::Id) -> Result<ByteSizedVec<u8>, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, get_value::Response>(get_value::Command::new(value_id))
            .await?
            .try_into()
    }

    async fn read_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute_id: u16,
        mask: u8,
        manufacturer_code: u16,
    ) -> Result<read_attribute::Attribbute, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, read_attribute::Response>(read_attribute::Command::new(
            endpoint,
            cluster,
            attribute_id,
            mask,
            manufacturer_code,
        ))
        .await?
        .try_into()
    }

    async fn send_pan_id_update(&mut self, new_pan: PanId) -> Result<bool, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, send_pan_id_update::Response>(send_pan_id_update::Command::new(
            new_pan,
        ))
        .await
        .map(Into::into)
    }

    async fn set_configuration_value(&mut self, config_id: Id, value: u16) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_configuration_value::Response>(
            set_configuration_value::Command::new(config_id, value),
        )
        .await?
        .try_into()
    }

    async fn set_passive_ack_config(
        &mut self,
        config: u8,
        min_acks_needed: u8,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_passive_ack_config::Response>(
            set_passive_ack_config::Command::new(config, min_acks_needed),
        )
        .await?
        .try_into()
    }

    async fn set_policy(
        &mut self,
        policy_id: policy::Id,
        decision_id: decision::Id,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_policy::Response>(set_policy::Command::new(
            policy_id,
            decision_id,
        ))
        .await?
        .try_into()
    }

    async fn set_value(
        &mut self,
        value_id: value::Id,
        value: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, set_value::Response>(set_value::Command::new(value_id, value))
            .await?
            .try_into()
    }

    async fn version(&mut self, desired_protocol_version: u8) -> Result<version::Response, Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, version::Response>(version::Command::new(desired_protocol_version))
            .await
    }

    async fn write_attribute(
        &mut self,
        endpoint: u8,
        cluster: u16,
        attribute: &Attribute,
        just_test: bool,
    ) -> Result<(), Error> {
        todo!();
        #[cfg(any())]
        self.communicate::<_, write_attribute::Response>(write_attribute::Command::new(
            endpoint, cluster, attribute, just_test,
        ))
        .await?
        .try_into()
    }
}
