use std::future::Future;

use crate::ember::aps::Frame;
use crate::ember::beacon::ClassificationParams;
use crate::ember::event::Units;
use crate::ember::message::Outgoing;
use crate::ember::multicast::TableEntry;
use crate::ember::{Eui64, NodeId};
use crate::error::Resolve;
use crate::frame::parameters::messaging::replace_address_table_entry::Payload;
use crate::frame::parameters::messaging::{
    address_table_entry_is_active, get_address_table_remote_eui64,
    get_address_table_remote_node_id, get_beacon_classification_params, get_extended_timeout,
    get_multicast_table_entry, lookup_eui64_by_node_id, lookup_node_id_by_eui64,
    maximum_payload_length, poll_for_data, proxy_broadcast, replace_address_table_entry,
    send_broadcast, send_many_to_one_route_request, send_multicast, send_multicast_with_alias,
    send_raw_message, send_raw_message_extended, send_reply, send_unicast,
    set_address_table_remote_eui64,
};
use crate::types::{ByteSizedVec, SourceRouteDiscoveryMode};
use crate::{Error, Transport};

pub trait Messaging {
    /// Indicates whether any messages are currently being sent using this address table entry.
    /// Note that this function does not indicate whether the address table entry is unused.
    /// To determine whether an address table entry is unused, check the remote node ID.
    /// The remote node ID will have the value `EMBER_TABLE_ENTRY_UNUSED_NODE_ID`
    /// when the address table entry is not in use.
    fn address_table_entry_is_active(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Gets the EUI64 of an address table entry.
    fn get_address_table_remote_eui64(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Gets the short ID of an address table entry.
    fn get_address_table_remote_node_id(
        &self,
        address_table_index: u8,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Gets the priority masks and related variables for choosing the best beacon.
    fn get_beacon_classification_params(
        &self,
    ) -> impl Future<Output = Result<ClassificationParams, Error>> + Send;

    /// Indicates whether the stack will extend the normal interval between retransmissions
    /// of a retried unicast message by `EMBER_INDIRECT_TRANSMISSION_TIMEOUT`.
    fn get_extended_timeout(
        &self,
        remote_eui64: Eui64,
    ) -> impl Future<Output = Result<bool, Error>> + Send;

    /// Gets an entry from the multicast table.
    fn get_multicast_table_entry(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Returns the EUI64 that corresponds to the specified node ID.
    ///
    /// The EUI64 is found by searching through all stack tables for the specified node ID.
    fn lookup_eui64_by_node_id(
        &self,
        node_id: NodeId,
    ) -> impl Future<Output = Result<Eui64, Error>> + Send;

    /// Returns the node ID that corresponds to the specified EUI64.
    ///
    /// The node ID is found by searching through all stack tables for the specified EUI64.
    fn lookup_node_id_by_eui64(
        &self,
        eui64: Eui64,
    ) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Returns the maximum size of the payload. The size depends on the security level in use.
    fn maximum_payload_length(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Periodically request any pending data from our parent.
    ///
    /// Setting interval to 0 or units to `EMBER_EVENT_INACTIVE` will generate a single poll.
    fn poll_for_data(
        &self,
        interval: u16,
        units: Units,
        failure_limit: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a proxied broadcast message as per the ZigBee specification.
    fn proxy_broadcast(
        &self,
        source: NodeId,
        destination: NodeId,
        nwk_sequence: u8,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    fn replace_address_table_entry(
        &self,
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> impl Future<Output = Result<replace_address_table_entry::Payload, Error>> + Send;

    /// Sends a broadcast message as per the ZigBee specification.
    fn send_broadcast(
        &self,
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
    /// referred to as an 'aggregator' in EmberZNet 2.x and earlier, and is referred to as a
    /// 'concentrator' in the ZigBee specification and EmberZNet 3.
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
    /// This function is only available in ZigBee Pro (stack profile 2), and cannot be called on
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
        &self,
        concentrator_type: u16,
        radius: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a multicast message to all endpoints that share a specific multicast ID and are
    /// within a specified number of hops of the sender.
    fn send_multicast(
        &self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Sends a multicast message to all endpoints that share a specific multicast ID and are
    /// within a specified number of hops of the sender.
    fn send_multicast_with_alias(
        &self,
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
        &self,
        message_contents: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Transmits the given message without modification.
    ///
    /// The MAC header is assumed to be configured in the message at the time this function is called.
    fn send_raw_message_extended(
        &self,
        message: ByteSizedVec<u8>,
        priority: u8,
        use_cca: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a reply to a received unicast message.
    ///
    /// The incomingMessageHandler callback for the unicast being replied to
    /// supplies the values for all the parameters except the reply itself.
    fn send_reply(
        &self,
        sender: NodeId,
        aps_frame: Frame,
        message: ByteSizedVec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a unicast message as per the ZigBee specification.
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
    /// If you want the packet to reach its destination, the APS_RETRY option must be set so that
    /// another attempt is made to transmit the message with its application payload after the route
    /// has been constructed.
    ///
    /// *Note*: When sending fragmented messages, the stack will only assign a new APS sequence number
    /// for the first fragment of the message (i.e., `EMBER_APS_OPTION_FRAGMENT` is set and the
    /// low-order byte of the groupId field in the APS frame is zero).
    /// For all subsequent fragments of the same message, the application must set the sequence
    /// number field in the APS frame to the sequence number assigned by the stack to the first fragment.
    fn send_unicast(
        &self,
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
        &self,
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
        &self,
        address_table_index: u8,
        id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the priority masks and related variables for choosing the best beacon.
    fn set_beacon_classification_params(
        &self,
        param: ClassificationParams,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Tells the stack whether the normal interval between retransmissions of a retried
    /// unicast message should be increased by EMBER_INDIRECT_TRANSMISSION_TIMEOUT.
    ///
    /// The interval needs to be increased when sending to a sleepy node so that the message is not
    /// retransmitted until the destination has had time to wake up and poll its parent.
    /// The stack will automatically extend the timeout:
    ///     * For our own sleepy children.
    ///     * When an address response is received from a parent on behalf of its child.
    ///     * When an indirect transaction expiry route error is received.
    ///     * When an end device announcement is received from a sleepy node.
    fn set_extended_timeout(
        &self,
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
        &self,
        wait_before_retry_interval_ms: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets an entry in the multicast table.
    fn set_multicast_table_entry(
        &self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets source route discovery (`MTORR`) mode to on, off, reschedule.
    fn set_source_route_discovery_mode(
        &self,
        mode: SourceRouteDiscoveryMode,
    ) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Send the network key to a destination.
    fn unicast_current_network_key(
        &self,
        target_short: NodeId,
        target_long: Eui64,
        parent_short_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Write the current node Id, PAN ID, or Node type to the tokens.
    fn write_node_data(&self, erase: bool) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Messaging for T
where
    T: Transport,
{
    async fn address_table_entry_is_active(&self, address_table_index: u8) -> Result<bool, Error> {
        self.communicate::<_, address_table_entry_is_active::Response>(
            address_table_entry_is_active::Command::new(address_table_index),
        )
        .await
        .map(|response| response.active())
    }

    async fn get_address_table_remote_eui64(
        &self,
        address_table_index: u8,
    ) -> Result<Eui64, Error> {
        self.communicate::<_, get_address_table_remote_eui64::Response>(
            get_address_table_remote_eui64::Command::new(address_table_index),
        )
        .await
        .map(|response| response.eui64())
    }

    async fn get_address_table_remote_node_id(
        &self,
        address_table_index: u8,
    ) -> Result<NodeId, Error> {
        self.communicate::<_, get_address_table_remote_node_id::Response>(
            get_address_table_remote_node_id::Command::new(address_table_index),
        )
        .await
        .map(|response| response.node_id())
    }

    async fn get_beacon_classification_params(&self) -> Result<ClassificationParams, Error> {
        self.communicate::<_, get_beacon_classification_params::Response>(
            get_beacon_classification_params::Command,
        )
        .await?
        .resolve()
    }

    async fn get_extended_timeout(&self, remote_eui64: Eui64) -> Result<bool, Error> {
        self.communicate::<_, get_extended_timeout::Response>(get_extended_timeout::Command::new(
            remote_eui64,
        ))
        .await
        .map(|response| response.extended_timeout())
    }

    async fn get_multicast_table_entry(&self, index: u8) -> Result<TableEntry, Error> {
        self.communicate::<_, get_multicast_table_entry::Response>(
            get_multicast_table_entry::Command::new(index),
        )
        .await?
        .resolve()
    }

    async fn lookup_eui64_by_node_id(&self, node_id: NodeId) -> Result<Eui64, Error> {
        self.communicate::<_, lookup_eui64_by_node_id::Response>(
            lookup_eui64_by_node_id::Command::new(node_id),
        )
        .await?
        .resolve()
    }

    async fn lookup_node_id_by_eui64(&self, eui64: Eui64) -> Result<NodeId, Error> {
        self.communicate::<_, lookup_node_id_by_eui64::Response>(
            lookup_node_id_by_eui64::Command::new(eui64),
        )
        .await
        .map(|response| response.node_id())
    }

    async fn maximum_payload_length(&self) -> Result<u8, Error> {
        self.communicate::<_, maximum_payload_length::Response>(maximum_payload_length::Command)
            .await
            .map(|response| response.aps_length())
    }

    async fn poll_for_data(
        &self,
        interval: u16,
        units: Units,
        failure_limit: u8,
    ) -> Result<(), Error> {
        self.communicate::<_, poll_for_data::Response>(poll_for_data::Command::new(
            interval,
            units,
            failure_limit,
        ))
        .await?
        .resolve()
    }

    async fn proxy_broadcast(
        &self,
        source: NodeId,
        destination: NodeId,
        nwk_sequence: u8,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        self.communicate::<_, proxy_broadcast::Response>(proxy_broadcast::Command::new(
            source,
            destination,
            nwk_sequence,
            aps_frame,
            radius,
            message_tag,
            message,
        ))
        .await?
        .resolve()
    }

    async fn replace_address_table_entry(
        &self,
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> Result<Payload, Error> {
        self.communicate::<_, replace_address_table_entry::Response>(
            replace_address_table_entry::Command::new(
                address_table_index,
                new_eui64,
                new_id,
                new_extended_timeout,
            ),
        )
        .await?
        .resolve()
    }

    async fn send_broadcast(
        &self,
        destination: NodeId,
        aps_frame: Frame,
        radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        self.communicate::<_, send_broadcast::Response>(send_broadcast::Command::new(
            destination,
            aps_frame,
            radius,
            message_tag,
            message,
        ))
        .await?
        .resolve()
    }

    async fn send_many_to_one_route_request(
        &self,
        concentrator_type: u16,
        radius: u8,
    ) -> Result<(), Error> {
        self.communicate::<_, send_many_to_one_route_request::Response>(
            send_many_to_one_route_request::Command::new(concentrator_type, radius),
        )
        .await?
        .resolve()
    }

    async fn send_multicast(
        &self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        self.communicate::<_, send_multicast::Response>(send_multicast::Command::new(
            aps_frame,
            hops,
            nonmember_radius,
            message_tag,
            message,
        ))
        .await?
        .resolve()
    }

    async fn send_multicast_with_alias(
        &self,
        aps_frame: Frame,
        hops: u8,
        nonmember_radius: u8,
        alias: u16,
        nwk_sequence: u8,
        message_tag: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        self.communicate::<_, send_multicast_with_alias::Response>(
            send_multicast_with_alias::Command::new(
                aps_frame,
                hops,
                nonmember_radius,
                alias,
                nwk_sequence,
                message_tag,
                message_contents,
            ),
        )
        .await?
        .resolve()
    }

    async fn send_raw_message(&self, message_contents: ByteSizedVec<u8>) -> Result<(), Error> {
        self.communicate::<_, send_raw_message::Response>(send_raw_message::Command::new(
            message_contents,
        ))
        .await?
        .resolve()
    }

    async fn send_raw_message_extended(
        &self,
        message: ByteSizedVec<u8>,
        priority: u8,
        use_cca: bool,
    ) -> Result<(), Error> {
        self.communicate::<_, send_raw_message_extended::Response>(
            send_raw_message_extended::Command::new(message, priority, use_cca),
        )
        .await?
        .resolve()
    }

    async fn send_reply(
        &self,
        sender: NodeId,
        aps_frame: Frame,
        message: ByteSizedVec<u8>,
    ) -> Result<(), Error> {
        self.communicate::<_, send_reply::Response>(send_reply::Command::new(
            sender, aps_frame, message,
        ))
        .await?
        .resolve()
    }

    async fn send_unicast(
        &self,
        typ: Outgoing,
        index_or_destination: NodeId,
        aps_frame: Frame,
        message_tag: u8,
        message: ByteSizedVec<u8>,
    ) -> Result<u8, Error> {
        self.communicate::<_, send_unicast::Response>(send_unicast::Command::new(
            typ,
            index_or_destination,
            aps_frame,
            message_tag,
            message,
        ))
        .await?
        .resolve()
    }

    async fn set_address_table_remote_eui64(
        &self,
        address_table_index: u8,
        eui64: Eui64,
    ) -> Result<(), Error> {
        self.communicate::<_, set_address_table_remote_eui64::Response>(
            set_address_table_remote_eui64::Command::new(address_table_index, eui64),
        )
        .await?
        .resolve()
    }

    fn set_address_table_remote_node_id(
        &self,
        address_table_index: u8,
        id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn set_beacon_classification_params(
        &self,
        param: ClassificationParams,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn set_extended_timeout(
        &self,
        remote_eui64: Eui64,
        extended_timeout: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn set_mac_poll_failure_wait_time(
        &self,
        wait_before_retry_interval_ms: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn set_multicast_table_entry(
        &self,
        index: u8,
        value: TableEntry,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn set_source_route_discovery_mode(
        &self,
        mode: SourceRouteDiscoveryMode,
    ) -> impl Future<Output = Result<u32, Error>> + Send {
        todo!()
    }

    fn unicast_current_network_key(
        &self,
        target_short: NodeId,
        target_long: Eui64,
        parent_short_id: NodeId,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }

    fn write_node_data(&self, erase: bool) -> impl Future<Output = Result<(), Error>> + Send {
        todo!()
    }
}
