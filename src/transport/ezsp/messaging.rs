use std::future::Future;

use crate::ember::aps::Frame;
use crate::ember::beacon::ClassificationParams;
use crate::ember::event::Units;
use crate::ember::message::Outgoing;
use crate::ember::multicast::TableEntry;
use crate::ember::{Eui64, NodeId};
use crate::error::Error;
use crate::frame::parameters::messaging::{
    address_table_entry_is_active, get_address_table_remote_eui64,
    get_address_table_remote_node_id, get_beacon_classification_params, get_extended_timeout,
    get_multicast_table_entry, lookup_eui64_by_node_id, lookup_node_id_by_eui64,
    maximum_payload_length, poll_for_data, proxy_broadcast, replace_address_table_entry,
    send_broadcast, send_many_to_one_route_request, send_multicast, send_multicast_with_alias,
    send_raw_message, send_raw_message_extended, send_reply, send_unicast,
    set_address_table_remote_eui64, set_address_table_remote_node_id,
    set_beacon_classification_params, set_extended_timeout, set_mac_poll_failure_wait_time,
    set_multicast_table_entry, set_source_route_discovery_mode, unicast_current_network_key,
    write_node_data,
};
use crate::transport::Transport;
use crate::types::{ByteSizedVec, SourceRouteDiscoveryMode};

/// The `Messaging` trait provides an interface for the messaging features.
pub trait Messaging {
    /// Indicates whether any messages are currently being sent using this address table entry.
    /// Note that this function does not indicate whether the address table entry is unused.
    /// To determine whether an address table entry is unused, check the remote node ID.
    /// The remote node ID will have the value `EMBER_TABLE_ENTRY_UNUSED_NODE_ID`
    /// when the address table entry is not in use.
    fn address_table_entry_is_active(
        &mut self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Gets the EUI64 of an address table entry.
    fn get_address_table_remote_eui64(
        &mut self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Gets the short ID of an address table entry.
    fn get_address_table_remote_node_id(
        &mut self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Gets the priority masks and related variables for choosing the best beacon.
    fn get_beacon_classification_params(
        &mut self,
    ) -> impl Future<Output = Result<ClassificationParams, Error>> + Send;

    /// Indicates whether the stack will extend the normal interval between retransmissions
    /// of a retried unicast message by `EMBER_INDIRECT_TRANSMISSION_TIMEOUT`.
    fn get_extended_timeout(
        &mut self,
        remote_eui64: Eui64,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Gets an entry from the multicast table.
    fn get_multicast_table_entry(
        &mut self,
        index: u8,
    ) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Returns the EUI64 that corresponds to the specified node ID.
    ///
    /// The EUI64 is found by searching through all stack tables for the specified node ID.
    fn lookup_eui64_by_node_id(
        &mut self,
        node_id: NodeId,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Returns the node ID that corresponds to the specified EUI64.
    ///
    /// The node ID is found by searching through all stack tables for the specified EUI64.
    fn lookup_node_id_by_eui64(
        &mut self,
        eui64: Eui64,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Returns the maximum size of the payload. The size depends on the security level in use.
    fn maximum_payload_length(&mut self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Periodically request any pending data from our parent.
    ///
    /// Setting interval to 0 or units to `EMBER_EVENT_INACTIVE` will generate a single poll.
    fn poll_for_data(
        &mut self,
        interval: u16,
        units: Units,
        failure_limit: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a proxied broadcast message as per the Zigbee specification.
    #[allow(clippy::too_many_arguments)]
    fn proxy_broadcast(
        &mut self,
        source: NodeId,
        destination: NodeId,
        nwk_sequence: u8,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Replaces the EUI64, short ID and extended timeout setting of an address table entry.
    ///
    /// The previous EUI64, short ID and extended timeout setting are returned.
    fn replace_address_table_entry(
        &mut self,
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> impl Future<Output = Result<replace_address_table_entry::PreviousEntry, Error>> + Send;

    /// Sends a broadcast message as per the Zigbee specification.
    fn send_broadcast(
        &mut self,
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Sends a route request packet that creates routes from every node in the network back to this node.
    ///
    /// This function should be called by an application that wishes to communicate with many nodes,
    /// for example, a gateway, central monitor, or controller. A device using this function was
    /// referred to as an 'aggregator' in `EmberZNet` 2.x and earlier, and is referred to as a
    /// 'concentrator' in the Zigbee specification and `EmberZNet` 3.
    ///
    /// This function enables large scale networks, because the other devices do not have to
    /// individually perform bandwidth-intensive route discoveries.
    /// Instead, when a remote node sends an APS unicast to a concentrator,
    /// its network layer automatically delivers a special route record packet first,
    /// which lists the network ids of all the intermediate relays.
    /// The concentrator can then use source routing to send outbound APS unicasts.
    /// (A source routed message is one in which the entire route is listed in the network layer header.)
    /// This allows the concentrator to communicate with thousands of devices without requiring
    /// large route tables on neighboring nodes.
    ///
    /// This function is only available in Zigbee Pro (stack profile 2), and cannot be called on
    /// end devices.
    /// Any router can be a concentrator (not just the coordinator),
    /// and there can be multiple concentrators on a network.
    ///
    /// Note that a concentrator does not automatically obtain routes to all network nodes after
    /// calling this function.
    /// Remote applications must first initiate an inbound APS unicast.
    ///
    /// Many-to-one routes are not repaired automatically.
    /// Instead, the concentrator application must call this function to rediscover the routes as
    /// necessary, for example, upon failure of a retried APS message.
    /// The reason for this is that there is no scalable one-size-fits-all route repair strategy.
    /// A common and recommended strategy is for the concentrator application to refresh the routes
    /// by calling this function periodically.
    fn send_many_to_one_route_request(
        &mut self,
        concentrator_type: u16,
        radius: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a multicast message to all endpoints that share a specific multicast ID and are
    /// within a specified number of hops of the sender.
    fn send_multicast(
        &mut self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Sends a multicast message to all endpoints that share a specific multicast ID and are
    /// within a specified number of hops of the sender.
    #[allow(clippy::too_many_arguments)]
    fn send_multicast_with_alias(
        &mut self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        alias: u16,
        nwk_sequence: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Transmits the given message without modification.
    ///
    /// The MAC header is assumed to be configured in the message at the time this function is called.
    fn send_raw_message(
        &mut self,
        message_contents: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Transmits the given message without modification.
    ///
    /// The MAC header is assumed to be configured in the message at the time this function is called.
    fn send_raw_message_extended(
        &mut self,
        message: ByteSizedVec<u8>,
        priority: u8,
        use_cca: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a reply to a received unicast message.
    ///
    /// The incomingMessageHandler callback for the unicast being replied to
    /// supplies the values for all the parameters except the reply itself.
    fn send_reply(
        &mut self,
        sender: NodeId,
        aps_frame: Frame,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a unicast message as per the Zigbee specification.
    ///
    /// The message will arrive at its destination only if there is a known route to the destination node.
    /// Setting the `ENABLE_ROUTE_DISCOVERY` option will cause a route to be discovered if none is known.
    /// Setting the `FORCE_ROUTE_DISCOVERY` option will force route discovery.
    /// Routes to end-device children of the local node are always known.
    /// Setting the `APS_RETRY` option will cause the message to be retransmitted until either a
    /// matching acknowledgement is received or three transmissions have been made.
    ///
    /// *Note*: Using the `FORCE_ROUTE_DISCOVERY` option will cause the first transmission to be
    /// consumed by a route request as part of discovery, so the application payload of this packet
    /// will not reach its destination on the first attempt.
    /// If you want the packet to reach its destination, the `APS_RETRY` option must be set so that
    /// another attempt is made to transmit the message with its application payload after the route
    /// has been constructed.
    ///
    /// *Note*: When sending fragmented messages, the stack will only assign a new APS sequence number
    /// for the first fragment of the message (i.e., `EMBER_APS_OPTION_FRAGMENT` is set and the
    /// low-order byte of the groupId field in the APS frame is zero).
    /// For all subsequent fragments of the same message, the application must set the sequence
    /// number field in the APS frame to the sequence number assigned by the stack to the first fragment.
    fn send_unicast(
        &mut self,
        typ: Outgoing,
        index_or_destination: NodeId,
        aps_frame: Frame,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Sets the EUI64 of an address table entry.
    ///
    /// This function will also check other address table entries, the child table and the neighbor
    /// table to see if the node ID for the given EUI64 is already known.
    /// If known then this function will also set node ID. If not known it will set the node ID to
    /// `EMBER_UNKNOWN_NODE_ID`.
    fn set_address_table_remote_eui64(
        &mut self,
        address_table_index: u8,
        eui64: Eui64,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the short ID of an address table entry.
    ///
    /// Usually the application will not need to set the short ID in the address table.
    /// Once the remote EUI64 is set the stack is capable of figuring out the short ID on its own.
    /// However, in cases where the application does set the short ID, the application must set the
    /// remote EUI64 prior to setting the short ID.
    fn set_address_table_remote_node_id(
        &mut self,
        address_table_index: u8,
        id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the priority masks and related variables for choosing the best beacon.
    fn set_beacon_classification_params(
        &mut self,
        param: ClassificationParams,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Tells the stack whether the normal interval between retransmissions of a retried
    /// unicast message should be increased by `EMBER_INDIRECT_TRANSMISSION_TIMEOUT`.
    ///
    /// The interval needs to be increased when sending to a sleepy node so that the message is not
    /// retransmitted until the destination has had time to wake up and poll its parent.
    /// The stack will automatically extend the timeout:
    ///     * For our own sleepy children.
    ///     * When an address response is received from a parent on behalf of its child.
    ///     * When an indirect transaction expiry route error is received.
    ///     * When an end device announcement is received from a sleepy node.
    fn set_extended_timeout(
        &mut self,
        remote_eui64: Eui64,
        extended_timeout: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function will set the retry interval (in milliseconds) for mac data poll.
    ///
    /// This interval is the time in milliseconds the device waits before retrying a data poll
    /// when a MAC level data poll fails for any reason.
    ///
    /// This function is useful to sleepy end devices.
    fn set_mac_poll_failure_wait_time(
        &mut self,
        wait_before_retry_interval_ms: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets an entry in the multicast table.
    fn set_multicast_table_entry(
        &mut self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets source route discovery (`MTORR`) mode to on, off, reschedule.
    fn set_source_route_discovery_mode(
        &mut self,
        mode: SourceRouteDiscoveryMode,
    ) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Send the network key to a destination.
    fn unicast_current_network_key(
        &mut self,
        target_short: NodeId,
        target_long: Eui64,
        parent_short_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Write the current node ID, PAN ID, or Node type to the tokens.
    fn write_node_data(&mut self, erase: bool) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Messaging for T
where
    T: Transport,
{
    async fn address_table_entry_is_active(
        &mut self,
        address_table_index: u8,
    ) -> Result<bool, Error> {
        Ok(address_table_entry_is_active::Response::try_from(
            self.communicate(address_table_entry_is_active::Command::new(
                address_table_index,
            ))
            .await?,
        )
        .map(|response| response.active())?)
    }

    async fn get_address_table_remote_eui64(
        &mut self,
        address_table_index: u8,
    ) -> Result<Eui64, Error> {
        Ok(get_address_table_remote_eui64::Response::try_from(
            self.communicate(get_address_table_remote_eui64::Command::new(
                address_table_index,
            ))
            .await?,
        )
        .map(|response| response.eui64())?)
    }

    async fn get_address_table_remote_node_id(
        &mut self,
        address_table_index: u8,
    ) -> Result<NodeId, Error> {
        Ok(get_address_table_remote_node_id::Response::try_from(
            self.communicate(get_address_table_remote_node_id::Command::new(
                address_table_index,
            ))
            .await?,
        )
        .map(|response| response.node_id())?)
    }

    async fn get_beacon_classification_params(&mut self) -> Result<ClassificationParams, Error> {
        get_beacon_classification_params::Response::try_from(
            self.communicate(get_beacon_classification_params::Command)
                .await?,
        )?
        .try_into()
    }

    async fn get_extended_timeout(&mut self, remote_eui64: Eui64) -> Result<bool, Error> {
        Ok(get_extended_timeout::Response::try_from(
            self.communicate(get_extended_timeout::Command::new(remote_eui64))
                .await?,
        )
        .map(|response| response.extended_timeout())?)
    }

    async fn get_multicast_table_entry(&mut self, index: u8) -> Result<TableEntry, Error> {
        get_multicast_table_entry::Response::try_from(
            self.communicate(get_multicast_table_entry::Command::new(index))
                .await?,
        )?
        .try_into()
    }

    async fn lookup_eui64_by_node_id(&mut self, node_id: NodeId) -> Result<Eui64, Error> {
        lookup_eui64_by_node_id::Response::try_from(
            self.communicate(lookup_eui64_by_node_id::Command::new(node_id))
                .await?,
        )?
        .try_into()
    }

    async fn lookup_node_id_by_eui64(&mut self, eui64: Eui64) -> Result<NodeId, Error> {
        Ok(lookup_node_id_by_eui64::Response::try_from(
            self.communicate(lookup_node_id_by_eui64::Command::new(eui64))
                .await?,
        )
        .map(|response| response.node_id())?)
    }

    async fn maximum_payload_length(&mut self) -> Result<u8, Error> {
        Ok(maximum_payload_length::Response::try_from(
            self.communicate(maximum_payload_length::Command).await?,
        )
        .map(|response| response.aps_length())?)
    }

    async fn poll_for_data(
        &mut self,
        interval: u16,
        units: Units,
        failure_limit: u8,
    ) -> Result<(), Error> {
        poll_for_data::Response::try_from(
            self.communicate(poll_for_data::Command::new(interval, units, failure_limit))
                .await?,
        )?
        .try_into()
    }

    async fn proxy_broadcast(
        &mut self,
        source: NodeId,
        destination: NodeId,
        nwk_sequence: u8,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        proxy_broadcast::Response::try_from(
            self.communicate(proxy_broadcast::Command::new(
                source,
                destination,
                nwk_sequence,
                aps_frame,
                radius,
                message_tag,
                message,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn replace_address_table_entry(
        &mut self,
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> Result<replace_address_table_entry::PreviousEntry, Error> {
        replace_address_table_entry::Response::try_from(
            self.communicate(replace_address_table_entry::Command::new(
                address_table_index,
                new_eui64,
                new_id,
                new_extended_timeout,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_broadcast(
        &mut self,
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        send_broadcast::Response::try_from(
            self.communicate(send_broadcast::Command::new(
                destination,
                aps_frame,
                radius,
                message_tag,
                message,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_many_to_one_route_request(
        &mut self,
        concentrator_type: u16,
        radius: u8,
    ) -> Result<(), Error> {
        send_many_to_one_route_request::Response::try_from(
            self.communicate(send_many_to_one_route_request::Command::new(
                concentrator_type,
                radius,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_multicast(
        &mut self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        send_multicast::Response::try_from(
            self.communicate(send_multicast::Command::new(
                aps_frame,
                hops,
                nonmember_radius,
                message_tag,
                message,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_multicast_with_alias(
        &mut self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        alias: u16,
        nwk_sequence: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        send_multicast_with_alias::Response::try_from(
            self.communicate(send_multicast_with_alias::Command::new(
                aps_frame,
                hops,
                nonmember_radius,
                alias,
                nwk_sequence,
                message_tag,
                message_contents,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_raw_message(&mut self, message_contents: ByteSizedVec<u8>) -> Result<(), Error> {
        send_raw_message::Response::try_from(
            self.communicate(send_raw_message::Command::new(message_contents))
                .await?,
        )?
        .try_into()
    }

    async fn send_raw_message_extended(
        &mut self,
        message: ByteSizedVec<u8>,
        priority: u8,
        use_cca: bool,
    ) -> Result<(), Error> {
        send_raw_message_extended::Response::try_from(
            self.communicate(send_raw_message_extended::Command::new(
                message, priority, use_cca,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn send_reply(
        &mut self,
        sender: NodeId,
        aps_frame: Frame,
        message: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        send_reply::Response::try_from(
            self.communicate(send_reply::Command::new(sender, aps_frame, message))
                .await?,
        )?
        .try_into()
    }

    async fn send_unicast(
        &mut self,
        typ: Outgoing,
        index_or_destination: NodeId,
        aps_frame: Frame,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        send_unicast::Response::try_from(
            self.communicate(send_unicast::Command::new(
                typ,
                index_or_destination,
                aps_frame,
                message_tag,
                message,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn set_address_table_remote_eui64(
        &mut self,
        address_table_index: u8,
        eui64: Eui64,
    ) -> Result<(), Error> {
        set_address_table_remote_eui64::Response::try_from(
            self.communicate(set_address_table_remote_eui64::Command::new(
                address_table_index,
                eui64,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn set_address_table_remote_node_id(
        &mut self,
        address_table_index: u8,
        id: NodeId,
    ) -> Result<(), Error> {
        Ok(set_address_table_remote_node_id::Response::try_from(
            self.communicate(set_address_table_remote_node_id::Command::new(
                address_table_index,
                id,
            ))
            .await?,
        )
        .map(drop)?)
    }

    async fn set_beacon_classification_params(
        &mut self,
        param: ClassificationParams,
    ) -> Result<(), Error> {
        set_beacon_classification_params::Response::try_from(
            self.communicate(set_beacon_classification_params::Command::new(param))
                .await?,
        )?
        .try_into()
    }

    async fn set_extended_timeout(
        &mut self,
        remote_eui64: Eui64,
        extended_timeout: bool,
    ) -> Result<(), Error> {
        Ok(set_extended_timeout::Response::try_from(
            self.communicate(set_extended_timeout::Command::new(
                remote_eui64,
                extended_timeout,
            ))
            .await?,
        )
        .map(drop)?)
    }

    async fn set_mac_poll_failure_wait_time(
        &mut self,
        wait_before_retry_interval_ms: u8,
    ) -> Result<(), Error> {
        Ok(set_mac_poll_failure_wait_time::Response::try_from(
            self.communicate(set_mac_poll_failure_wait_time::Command::new(
                wait_before_retry_interval_ms,
            ))
            .await?,
        )
        .map(drop)?)
    }

    async fn set_multicast_table_entry(
        &mut self,
        index: u8,
        value: TableEntry,
    ) -> Result<(), Error> {
        set_multicast_table_entry::Response::try_from(
            self.communicate(set_multicast_table_entry::Command::new(index, value))
                .await?,
        )?
        .try_into()
    }

    async fn set_source_route_discovery_mode(
        &mut self,
        mode: SourceRouteDiscoveryMode,
    ) -> Result<u32, Error> {
        Ok(set_source_route_discovery_mode::Response::try_from(
            self.communicate(set_source_route_discovery_mode::Command::new(mode))
                .await?,
        )
        .map(|response| response.remaining_time())?)
    }

    async fn unicast_current_network_key(
        &mut self,
        target_short: NodeId,
        target_long: Eui64,
        parent_short_id: NodeId,
    ) -> Result<(), Error> {
        unicast_current_network_key::Response::try_from(
            self.communicate(unicast_current_network_key::Command::new(
                target_short,
                target_long,
                parent_short_id,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn write_node_data(&mut self, erase: bool) -> Result<(), Error> {
        write_node_data::Response::try_from(
            self.communicate(write_node_data::Command::new(erase))
                .await?,
        )?
        .try_into()
    }
}
