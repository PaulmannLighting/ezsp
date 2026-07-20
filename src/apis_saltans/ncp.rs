//! `apis_saltans_hw::Driver` adapter for [`Ncp`].
//!
//! [`ZigbeeNcp`] wraps the plain host-side [`Ncp`] and registers every supplied
//! endpoint as part of construction. Consequently, a value that implements
//! `apis_saltans_hw::Driver` always has the same endpoints configured on the
//! device and available through `Driver::get_endpoints`.
//! Endpoint registration is implemented by the private [`AddSimpleDescriptors`]
//! extension trait, which translates the `apis-saltans` descriptors into
//! sequential [`Ncp::add_endpoint`] calls.
//!
//! The adapter maps the generic hardware-driver operations expected by
//! `apis-saltans` to EZSP command traits. Direct NCP operations are forwarded to
//! the wrapped transport, while APS sends use the wrapped [`Ncp`] so that EZSP
//! `messageSent` callbacks can be correlated with the originating request. The
//! APS profile and cluster are taken from `apis_saltans_hw::Datagram` metadata;
//! the local source endpoint is selected by [`Ncp`] from the registered
//! endpoint output clusters.

use std::time::Duration;

use apis_saltans_hw::core::{Destination, IeeeAddress};
use apis_saltans_hw::zdp::SimpleDescriptor;
use apis_saltans_hw::{Datagram, Driver, Error, FoundNetwork, HwResponse, ScannedChannel};

use crate::ember::concentrator;
use crate::{Configuration, Messaging, MulticastOptions, Ncp, Networking, Utilities};

mod builder;
mod event_handler;

const DEFAULT_BROADCAST_RADIUS: u8 = 0;
const DEFAULT_MULTICAST_HOPS: u8 = 0;
const DEFAULT_MULTICAST_NONMEMBER_RADIUS: u8 = 0;

/// An `apis-saltans` driver backed by a fully configured [`Ncp`].
///
/// This newtype is created only after all supplied endpoint descriptors have
/// been registered with the physical NCP. It retains those descriptors for
/// [`Driver::get_endpoints`], while the wrapped [`Ncp`] retains their output
/// clusters for APS source-endpoint selection.
pub struct ZigbeeNcp<T> {
    ncp: Ncp<T>,
    endpoints: Box<[SimpleDescriptor]>,
}

impl<T> ZigbeeNcp<T>
where
    T: Configuration + Send,
{
    /// Registers `endpoints` with `ncp` and wraps the configured NCP.
    ///
    /// Descriptors are assigned endpoint numbers starting at one, in slice
    /// order. The wrapper is returned only after every endpoint registration
    /// succeeds, ensuring that a constructed [`ZigbeeNcp`] is ready to serve as
    /// a [`Driver`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the EZSP command for any endpoint fails.
    pub(crate) async fn new(
        mut ncp: Ncp<T>,
        endpoints: Box<[SimpleDescriptor]>,
    ) -> Result<Self, Error> {
        ncp.add_simple_descriptors(&endpoints)
            .await
            .map(|()| Self { ncp, endpoints })
    }
}

impl<T> Driver for ZigbeeNcp<T>
where
    T: Configuration + Messaging + Networking + Utilities + Send + Sync,
{
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        Ok(self.endpoints.clone())
    }

    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(self.ncp.transport.get_node_id().await?)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(self.ncp.transport.get_eui64().await?.into())
    }

    async fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<FoundNetwork>, Error> {
        self.ncp
            .scan_networks(channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<ScannedChannel>, Error> {
        self.ncp
            .scan_channels(channel_mask, duration)
            .await
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        let seconds = u8::try_from(duration.as_secs()).unwrap_or(u8::MAX);
        self.ncp.transport.permit_joining(seconds.into()).await?;
        Ok(Duration::from_secs(u64::from(seconds)))
    }

    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        Ok(self
            .ncp
            .transport
            .send_many_to_one_route_request(concentrator::Type::HighRam, radius)
            .await?)
    }

    async fn short_id_to_ieee_address(&mut self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(self
            .ncp
            .transport
            .lookup_eui64_by_node_id(short_id)
            .await?
            .into())
    }

    async fn ieee_address_to_short_id(&mut self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(self
            .ncp
            .transport
            .lookup_node_id_by_eui64(ieee_address.into())
            .await?)
    }

    async fn transmit(
        &mut self,
        destination: Destination,
        datagram: Datagram,
    ) -> Result<HwResponse, Error> {
        let (metadata, payload) = datagram.into_parts();
        let profile = metadata.profile();
        let profile_id = profile.into();
        let cluster_id = metadata.cluster_id();

        let stack_response = match destination {
            Destination::Device(device) => {
                self.ncp
                    .unicast(
                        device.device().into(),
                        profile_id,
                        cluster_id,
                        device.endpoint().into(),
                        payload,
                    )
                    .await?
            }
            Destination::Broadcast(broadcast) => {
                self.ncp
                    .broadcast(
                        broadcast.address().as_u16(),
                        DEFAULT_BROADCAST_RADIUS,
                        profile_id,
                        cluster_id,
                        broadcast.endpoint().into(),
                        payload,
                    )
                    .await?
            }
            Destination::Group(group_id) => {
                let (stack_response, _seq) = self
                    .ncp
                    .multicast(
                        group_id.as_u16(),
                        MulticastOptions::new(
                            DEFAULT_MULTICAST_HOPS,
                            DEFAULT_MULTICAST_NONMEMBER_RADIUS,
                        ),
                        profile_id,
                        cluster_id,
                        profile.broadcast_endpoint().into(),
                        payload,
                    )
                    .await?;
                stack_response
            }
        };

        Ok(HwResponse::new(stack_response))
    }
}

/// Configures an [`Ncp`] from `apis-saltans` simple descriptors.
///
/// This private extension keeps descriptor translation and sequential endpoint
/// registration separate from [`ZigbeeNcp`] construction. Its returned future
/// is [`Send`] so construction remains compatible with the driver actor's
/// asynchronous startup path.
trait AddSimpleDescriptors {
    /// Registers `descriptors` with the device and the [`Ncp`] endpoint cache.
    ///
    /// Descriptor positions are mapped to endpoint numbers starting at one.
    /// Registrations are performed sequentially and stop at the first error.
    /// [`Ncp::add_endpoint`] updates its local output-cluster metadata only
    /// after the corresponding EZSP command succeeds.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any endpoint cannot be registered with the NCP.
    fn add_simple_descriptors(
        &mut self,
        descriptors: &[SimpleDescriptor],
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> AddSimpleDescriptors for Ncp<T>
where
    T: Configuration + Send,
{
    async fn add_simple_descriptors(
        &mut self,
        descriptors: &[SimpleDescriptor],
    ) -> Result<(), Error> {
        for (endpoint_id, descriptor) in
            descriptors
                .iter()
                .enumerate()
                .map_while(|(index, descriptor)| {
                    u8::try_from(index.checked_add(1)?)
                        .ok()
                        .map(|endpoint| (endpoint, descriptor))
                })
        {
            self.add_endpoint(
                endpoint_id,
                descriptor.profile_id(),
                descriptor.device_id(),
                descriptor.app_flags(),
                descriptor.input_clusters().iter().copied().collect(),
                descriptor.output_clusters().iter().copied().collect(),
            )
            .await?;
        }

        Ok(())
    }
}
