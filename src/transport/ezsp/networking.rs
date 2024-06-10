use std::future::Future;

use crate::ember::multi_phy::{nwk, radio};
use crate::ember::{
    beacon, child, duty_cycle, neighbor, network, node, route, DeviceDutyCycles, Eui64, NodeId,
};
use crate::error::Resolve;
use crate::ezsp::network::InitBitmask;
use crate::frame::parameters::networking::{
    child_id, clear_stored_beacons, energy_scan_request, find_and_rejoin_network,
    find_unused_pan_id, form_network, get_child_data, get_current_duty_cycle,
    get_duty_cycle_limits, get_duty_cycle_state, get_first_beacon, get_neighbor,
    get_neighbor_frame_counter, get_network_parameters, get_next_beacon, get_num_stored_beacons,
    get_parent_child_parameters, get_radio_channel, get_radio_parameters, get_route_table_entry,
    get_routing_shortcut_threshold, get_source_route_table_entry,
    get_source_route_table_filled_size, get_source_route_table_total_size, id, join_network,
    join_network_directly, leave_network, multi_phy_set_radio_channel, multi_phy_set_radio_power,
    multi_phy_start, multi_phy_stop, neighbor_count, network_init,
};
use crate::{Error, Transport};

/// Networking frames.
pub trait Networking {
    /// Convert a child index to a node ID.
    fn child_id(&self, child_index: u8) -> impl Future<Output = Result<NodeId, Error>> + Send;

    /// Clears all cached beacons that have been collected from a scan.
    fn clear_stored_beacons(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sends a ZDO energy scan request.
    ///
    /// This request may only be sent by the current network manager and must be unicast, not broadcast.
    fn energy_scan_request(
        &self,
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// The application may call this function when contact with the network has been lost.
    ///
    /// The most common usage case is when an end device can no longer communicate with its parent
    /// and wishes to find a new one. Another case is when a device has missed a Network Key update
    /// and no longer has the current Network Key.
    ///
    /// The stack will call `ezspStackStatusHandler` to indicate that the network is down,
    /// then try to re-establish contact with the network by performing an active scan,
    /// choosing a network with matching extended pan id, and sending a ZigBee network rejoin request.
    /// A second call to the `ezspStackStatusHandler` callback indicates either the success or the
    /// failure of the attempt. The process takes approximately 150 milliseconds per channel to complete.
    ///
    /// This call replaces the emberMobileNodeHasMoved API from EmberZNet 2.x,
    /// which used MAC association and consequently took half a second longer to complete.
    fn find_and_rejoin_network(
        &self,
        have_current_network_key: bool,
        channel_mask: u32,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This function starts a series of scans which will return an available panId.
    fn find_unused_pan_id(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Forms a new network by becoming the coordinator.
    fn form_network(
        &self,
        parameters: network::Parameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Returns information about a child of the local node.
    fn get_child_data(&self, index: u8) -> impl Future<Output = Result<child::Data, Error>> + Send;

    /// Returns the duty cycle of the stack's connected children that are being monitored, up to `max_devices`.
    ///
    /// It indicates the amount of overall duty cycle they have consumed (up to the suspend limit).
    /// The first entry is always the local stack's nodeId, and thus the total aggregate duty cycle
    /// for the device. The passed pointer arrayOfDeviceDutyCycles MUST have space for `max_devices`.
    fn get_current_duty_cycle(
        &self,
        max_devices: u8,
    ) -> impl Future<Output = Result<DeviceDutyCycles, Error>> + Send;

    /// Obtains the current duty cycle limits that were previously set by a call to
    /// [`set_duty_cycle_limits_in_stack()`](Self::set_duty_cycle_limits_in_stack),
    /// or the defaults set by the stack if no set call was made.
    fn get_duty_cycle_limits(
        &self,
    ) -> impl Future<Output = Result<duty_cycle::Limits, Error>> + Send;

    /// Obtains the current duty cycle state.
    fn get_duty_cycle_state(&self)
        -> impl Future<Output = Result<duty_cycle::State, Error>> + Send;

    /// Returns the first beacon in the cache.
    ///
    /// Beacons are stored in cache after issuing an active scan.
    fn get_first_beacon(&self) -> impl Future<Output = Result<beacon::Iterator, Error>> + Send;

    /// Returns the neighbor table entry at the given index.
    ///
    /// The number of active neighbors can be obtained using the
    /// [`neighbor_count()`](Self::neighbor_count) command.
    fn get_neighbor(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<neighbor::TableEntry, Error>> + Send;

    /// Return counter status depending on whether the frame counter of the node is found in the
    /// neighbor or child table.
    ///
    /// This function gets the last received frame counter as found in the Network Auxiliary header
    /// for the specified neighbor or child
    fn get_neighbor_frame_counter(
        &self,
        eui64: Eui64,
    ) -> impl Future<Output = Result<u32, Error>> + Send;

    /// Returns the current network parameters.
    fn get_network_parameters(
        &self,
    ) -> impl Future<Output = Result<(node::Type, network::Parameters), Error>> + Send;

    /// Returns the next beacon in the cache.
    ///
    /// Beacons are stored in cache after issuing an active scan.
    fn get_next_beacon(&self) -> impl Future<Output = Result<beacon::Data, Error>> + Send;

    /// Returns the number of cached beacons that have been collected from a scan.
    fn get_num_stored_beacons(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns information about the children of the local node and the parent of the local node.
    fn get_parent_child_parameters(
        &self,
    ) -> impl Future<Output = Result<get_parent_child_parameters::Response, Error>> + Send;

    /// Gets the channel in use for sending and receiving messages.
    fn get_radio_channel(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns the current radio parameters based on phy index.
    fn get_radio_parameters(
        &self,
        phy_index: u8,
    ) -> impl Future<Output = Result<radio::Parameters, Error>> + Send;

    /// Returns the route table entry at the given index.
    ///
    /// The route table size can be obtained using the
    /// [`get_configuration_value()`](Self::get_configuration_value) command.
    fn get_route_table_entry(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<route::TableEntry, Error>> + Send;

    /// Gets the routing shortcut threshold used to differentiate between directly using a neighbor
    /// vs. performing routing.
    fn get_routing_shortcut_threshold(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns information about a source route table entry.
    fn get_source_route_table_entry(
        &self,
        index: u8,
    ) -> impl Future<Output = Result<(NodeId, u8), Error>> + Send;

    /// Returns the number of filled entries in source route table.
    fn get_source_route_table_filled_size(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Returns the source route table total size.
    fn get_source_route_table_total_size(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Convert a node ID to a child index.
    fn id(&self, child_id: NodeId) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Causes the stack to associate with the network using the specified network parameters.
    ///
    /// It can take several seconds for the stack to associate with the local network.
    /// Do not send messages until the stackStatusHandler callback informs you that the stack is up.
    fn join_network(
        &self,
        node_type: node::Type,
        parameters: network::Parameters,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Causes the stack to associate with the network using the specified network parameters in
    /// the beacon parameter.
    ///
    /// It can take several seconds for the stack to associate with the local network.
    /// Do not send messages until the stackStatusHandler callback informs you that the stack is up.
    /// Unlike ::emberJoinNetwork(), this function does not issue an active scan before joining.
    /// Instead, it will cause the local node to issue a MAC Association Request directly to the
    /// specified target node. It is assumed that the beacon parameter is an artifact after issuing
    /// an active scan. (For more information, see emberGetBestBeacon and emberGetNextBeacon.)
    fn join_network_directly(
        &self,
        local_node_type: node::Type,
        beacon: beacon::Data,
        radio_tx_power: i8,
        clear_beacons_after_network_up: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Causes the stack to leave the current network.
    ///
    /// This generates a stackStatusHandler callback to indicate that the network is down.
    /// The radio will not be used until after sending a formNetwork or joinNetwork command.
    fn leave_network(&self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the channel for desired phy interface to use for sending and receiving messages.
    ///
    /// For a list of available radio pages and channels, see the technical specification for the
    /// RF communication module in your Developer Kit.
    ///
    /// Note: Care should be taken when using this API,
    /// as all devices on a network must use the same page and channel.
    fn multi_phy_set_radio_channel(
        &self,
        phy_index: u8,
        page: u8,
        channel: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Sets the radio output power for desired phy interface at which a node is operating.
    ///
    /// Ember radios have discrete power settings. For a list of available power settings,
    /// see the technical specification for the RF communication module in your Developer Kit.
    ///
    /// Note: Care should be taken when using this api on a running network,
    /// as it will directly impact the established link qualities neighboring
    /// nodes have with the node on which it is called.
    /// This can lead to disruption of existing routes and erratic network behavior.
    fn multi_phy_set_radio_power(
        &self,
        phy_index: u8,
        power: i8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This causes to initialize the desired radio interface other than native and form a new
    /// network by becoming the coordinator with same panId as native radio network.
    fn multi_phy_start(
        &self,
        phy_index: u8,
        page: u8,
        channel: u8,
        power: i8,
        bitmask: nwk::Config,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// This causes to bring down the radio interface other than native.
    fn multi_phy_stop(&self, phy_index: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Returns the number of active entries in the neighbor table.
    fn neighbor_count(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Resume network operation after a reboot.
    ///
    /// The node retains its original type.
    /// This should be called on startup whether the node was previously part of a network.
    /// [`Status::NotJoined`](crate::ember::Status::NotJoined) is returned if the node is not part of a network.
    /// This command accepts options to control the network initialization.
    fn network_init(
        &self,
        bitmask: &[InitBitmask],
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Networking for T
where
    T: Transport,
{
    async fn child_id(&self, child_index: u8) -> Result<NodeId, Error> {
        self.communicate::<_, child_id::Response>(child_id::Command::new(child_index))
            .await
            .map(|response| response.child_id())
    }

    async fn clear_stored_beacons(&self) -> Result<(), Error> {
        self.communicate::<_, clear_stored_beacons::Response>(clear_stored_beacons::Command)
            .await
            .map(drop)
    }

    async fn energy_scan_request(
        &self,
        target: NodeId,
        scan_channels: u32,
        scan_duration: u8,
        scan_count: u16,
    ) -> Result<(), Error> {
        self.communicate::<_, energy_scan_request::Response>(energy_scan_request::Command::new(
            target,
            scan_channels,
            scan_duration,
            scan_count,
        ))
        .await?
        .resolve()
    }

    async fn find_and_rejoin_network(
        &self,
        have_current_network_key: bool,
        channel_mask: u32,
    ) -> Result<(), Error> {
        self.communicate::<_, find_and_rejoin_network::Response>(
            find_and_rejoin_network::Command::new(have_current_network_key, channel_mask),
        )
        .await?
        .resolve()
    }

    async fn find_unused_pan_id(&self, channel_mask: u32, duration: u8) -> Result<(), Error> {
        self.communicate::<_, find_unused_pan_id::Response>(find_unused_pan_id::Command::new(
            channel_mask,
            duration,
        ))
        .await?
        .resolve()
    }

    async fn form_network(&self, parameters: network::Parameters) -> Result<(), Error> {
        self.communicate::<_, form_network::Response>(form_network::Command::new(parameters))
            .await?
            .resolve()
    }

    async fn get_child_data(&self, index: u8) -> Result<child::Data, Error> {
        self.communicate::<_, get_child_data::Response>(get_child_data::Command::new(index))
            .await?
            .resolve()
    }

    async fn get_current_duty_cycle(&self, max_devices: u8) -> Result<DeviceDutyCycles, Error> {
        self.communicate::<_, get_current_duty_cycle::Response>(
            get_current_duty_cycle::Command::new(max_devices),
        )
        .await?
        .resolve()
    }

    async fn get_duty_cycle_limits(&self) -> Result<duty_cycle::Limits, Error> {
        self.communicate::<_, get_duty_cycle_limits::Response>(get_duty_cycle_limits::Command)
            .await?
            .resolve()
    }

    async fn get_duty_cycle_state(&self) -> Result<duty_cycle::State, Error> {
        self.communicate::<_, get_duty_cycle_state::Response>(get_duty_cycle_state::Command)
            .await?
            .resolve()
    }

    async fn get_first_beacon(&self) -> Result<beacon::Iterator, Error> {
        self.communicate::<_, get_first_beacon::Response>(get_first_beacon::Command)
            .await?
            .resolve()
    }

    async fn get_neighbor(&self, index: u8) -> Result<neighbor::TableEntry, Error> {
        self.communicate::<_, get_neighbor::Response>(get_neighbor::Command::new(index))
            .await?
            .resolve()
    }

    async fn get_neighbor_frame_counter(&self, eui64: Eui64) -> Result<u32, Error> {
        self.communicate::<_, get_neighbor_frame_counter::Response>(
            get_neighbor_frame_counter::Command::new(eui64),
        )
        .await?
        .resolve()
    }

    async fn get_network_parameters(&self) -> Result<(node::Type, network::Parameters), Error> {
        self.communicate::<_, get_network_parameters::Response>(get_network_parameters::Command)
            .await?
            .resolve()
    }

    async fn get_next_beacon(&self) -> Result<beacon::Data, Error> {
        self.communicate::<_, get_next_beacon::Response>(get_next_beacon::Command)
            .await?
            .resolve()
    }

    async fn get_num_stored_beacons(&self) -> Result<u8, Error> {
        self.communicate::<_, get_num_stored_beacons::Response>(get_num_stored_beacons::Command)
            .await
            .map(|response| response.num_beacons())
    }

    async fn get_parent_child_parameters(
        &self,
    ) -> Result<get_parent_child_parameters::Response, Error> {
        self.communicate::<_, get_parent_child_parameters::Response>(
            get_parent_child_parameters::Command,
        )
        .await
    }

    async fn get_radio_channel(&self) -> Result<u8, Error> {
        self.communicate::<_, get_radio_channel::Response>(get_radio_channel::Command)
            .await
            .map(|response| response.channel())
    }

    async fn get_radio_parameters(&self, phy_index: u8) -> Result<radio::Parameters, Error> {
        self.communicate::<_, get_radio_parameters::Response>(get_radio_parameters::Command::new(
            phy_index,
        ))
        .await?
        .resolve()
    }

    async fn get_route_table_entry(&self, index: u8) -> Result<route::TableEntry, Error> {
        self.communicate::<_, get_route_table_entry::Response>(get_route_table_entry::Command::new(
            index,
        ))
        .await?
        .resolve()
    }

    async fn get_routing_shortcut_threshold(&self) -> Result<u8, Error> {
        self.communicate::<_, get_routing_shortcut_threshold::Response>(
            get_routing_shortcut_threshold::Command,
        )
        .await
        .map(|response| response.routing_shortcut_thresh())
    }

    async fn get_source_route_table_entry(&self, index: u8) -> Result<(NodeId, u8), Error> {
        self.communicate::<_, get_source_route_table_entry::Response>(
            get_source_route_table_entry::Command::new(index),
        )
        .await?
        .resolve()
    }

    async fn get_source_route_table_filled_size(&self) -> Result<u8, Error> {
        self.communicate::<_, get_source_route_table_filled_size::Response>(
            get_source_route_table_filled_size::Command,
        )
        .await
        .map(|response| response.source_route_table_filled_size())
    }

    async fn get_source_route_table_total_size(&self) -> Result<u8, Error> {
        self.communicate::<_, get_source_route_table_total_size::Response>(
            get_source_route_table_total_size::Command,
        )
        .await
        .map(|response| response.source_route_table_total_size())
    }

    async fn id(&self, child_id: NodeId) -> Result<u8, Error> {
        self.communicate::<_, id::Response>(id::Command::new(child_id))
            .await
            .map(|response| response.child_index())
    }

    async fn join_network(
        &self,
        node_type: node::Type,
        parameters: network::Parameters,
    ) -> Result<(), Error> {
        self.communicate::<_, join_network::Response>(join_network::Command::new(
            node_type, parameters,
        ))
        .await?
        .resolve()
    }

    async fn join_network_directly(
        &self,
        local_node_type: node::Type,
        beacon: beacon::Data,
        radio_tx_power: i8,
        clear_beacons_after_network_up: bool,
    ) -> Result<(), Error> {
        self.communicate::<_, join_network_directly::Response>(join_network_directly::Command::new(
            local_node_type,
            beacon,
            radio_tx_power,
            clear_beacons_after_network_up,
        ))
        .await?
        .resolve()
    }

    async fn leave_network(&self) -> Result<(), Error> {
        self.communicate::<_, leave_network::Response>(leave_network::Command)
            .await?
            .resolve()
    }

    async fn multi_phy_set_radio_channel(
        &self,
        phy_index: u8,
        page: u8,
        channel: u8,
    ) -> Result<(), Error> {
        self.communicate::<_, multi_phy_set_radio_channel::Response>(
            multi_phy_set_radio_channel::Command::new(phy_index, page, channel),
        )
        .await?
        .resolve()
    }

    async fn multi_phy_set_radio_power(&self, phy_index: u8, power: i8) -> Result<(), Error> {
        self.communicate::<_, multi_phy_set_radio_power::Response>(
            multi_phy_set_radio_power::Command::new(phy_index, power),
        )
        .await?
        .resolve()
    }

    async fn multi_phy_start(
        &self,
        phy_index: u8,
        page: u8,
        channel: u8,
        power: i8,
        bitmask: nwk::Config,
    ) -> Result<(), Error> {
        self.communicate::<_, multi_phy_start::Response>(multi_phy_start::Command::new(
            phy_index, page, channel, power, bitmask,
        ))
        .await?
        .resolve()
    }

    async fn multi_phy_stop(&self, phy_index: u8) -> Result<(), Error> {
        self.communicate::<_, multi_phy_stop::Response>(multi_phy_stop::Command::new(phy_index))
            .await?
            .resolve()
    }

    async fn neighbor_count(&self) -> Result<u8, Error> {
        self.communicate::<_, neighbor_count::Response>(neighbor_count::Command)
            .await
            .map(|response| response.value())
    }

    async fn network_init(&self, bitmask: &[InitBitmask]) -> Result<(), Error> {
        self.communicate::<_, network_init::Response>(network_init::Command::new(bitmask))
            .await?
            .resolve()
    }
}
