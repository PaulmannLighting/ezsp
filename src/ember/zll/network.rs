use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::node::Type;
use crate::ember::zll::{SecurityAlgorithmData, State};
use crate::ember::{zigbee, Eui64, NodeId};

/// The parameters of a ZLL network.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Network {
    zigbee_network: zigbee::Network,
    security_algorithm: SecurityAlgorithmData,
    eui64: Eui64,
    node_id: NodeId,
    state: u16,
    node_type: u8,
    number_sub_devices: u8,
    total_group_identifiers: u8,
    rssi_correction: u8,
}

impl Network {
    /// Create a new ZLL network.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        zigbee_network: zigbee::Network,
        security_algorithm: SecurityAlgorithmData,
        eui64: Eui64,
        node_id: NodeId,
        state: State,
        node_type: Type,
        number_sub_devices: u8,
        total_group_identifiers: u8,
        rssi_correction: u8,
    ) -> Self {
        Self {
            zigbee_network,
            security_algorithm,
            eui64,
            node_id,
            state: state.into(),
            node_type: node_type.into(),
            number_sub_devices,
            total_group_identifiers,
            rssi_correction,
        }
    }

    /// Return the parameters of a ZigBee network.
    #[must_use]
    pub const fn zigbee_network(&self) -> &zigbee::Network {
        &self.zigbee_network
    }

    /// Return the data associated with the ZLL security algorithm.
    #[must_use]
    pub const fn security_algorithm(&self) -> &SecurityAlgorithmData {
        &self.security_algorithm
    }

    /// Return the associated EUI64.
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    /// Return the node id.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the ZLL state.
    ///
    /// # Errors
    /// Returns the [`u16`] value of the state if it has an invalid value.
    pub fn state(&self) -> Result<State, u16> {
        State::try_from(self.state)
    }

    /// Return the node type.
    ///
    /// # Errors
    /// Returns the [`u8`] value of the type if it has an invalid value.
    pub fn node_type(&self) -> Result<Type, u8> {
        Type::try_from(self.node_type)
    }

    /// Return the number of sub devices.
    #[must_use]
    pub const fn number_sub_devices(&self) -> u8 {
        self.number_sub_devices
    }

    /// Return the total number of group identifiers.
    #[must_use]
    pub const fn total_group_identifiers(&self) -> u8 {
        self.total_group_identifiers
    }

    /// Return the RSSI correction value.
    #[must_use]
    pub const fn rssi_correction(&self) -> u8 {
        self.rssi_correction
    }
}
