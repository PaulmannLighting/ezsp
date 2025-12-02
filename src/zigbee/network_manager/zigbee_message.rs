use macaddr::MacAddr8;

use crate::ember::device::Update;
use crate::ember::node;
use crate::parameters::messaging::handler::IncomingMessage;
use crate::parameters::networking::handler::ChildJoin;
use crate::parameters::trust_center::handler::TrustCenterJoin;
use crate::types::ByteSizedVec;
use crate::zigbee::network_manager::address::Address;

/// Messages received through events within the Zigbee network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZigbeeMessage {
    /// A child device has joined the network.
    ChildJoined {
        /// The MAC address of the child device.
        id: MacAddr8,
        /// The short ID of the child device, if available.
        short_id: Option<u16>,
        /// The index of the child device in the child table.
        index: Option<u8>,
        /// The type of the child device, if available.
        typ: Option<node::Type>,
    },
    /// A child device has left the network.
    ChildLeft {
        /// The MAC address of the child device.
        id: MacAddr8,
        /// The short ID of the child device, if available.
        short_id: Option<u16>,
        /// The index of the child device in the child table.
        index: Option<u8>,
        /// The type of the child device, if available.
        typ: Option<node::Type>,
    },
    /// A device has joined the network through the Trust Center.
    TrustCenterJoin {
        /// The MAC address of the device.
        id: MacAddr8,
        /// The short ID of the device, if available.
        short_id: Option<u16>,
    },
    /// A device has rejoined the network through the Trust Center.
    TrustCenterRejoin {
        /// The MAC address of the device.
        id: MacAddr8,
        /// The short ID of the device, if available.
        short_id: Option<u16>,
        secure: bool,
    },
    /// A device has left the network through the Trust Center.
    TrustCenterLeave {
        /// The MAC address of the device.
        id: MacAddr8,
        /// The short ID of the device, if available.
        short_id: Option<u16>,
    },
    /// An incoming message from another device.
    IncomingMessage {
        /// The source address of the message.
        source: Address,
        /// The APS frame of the message.
        data: ByteSizedVec<u8>,
    },
}

impl From<ChildJoin> for ZigbeeMessage {
    fn from(value: ChildJoin) -> Self {
        if value.joining() {
            Self::ChildJoined {
                id: value.child_eui64(),
                short_id: Some(value.child_id()),
                index: Some(value.index()),
                typ: value.child_type().ok(),
            }
        } else {
            Self::ChildLeft {
                id: value.child_eui64(),
                short_id: Some(value.child_id()),
                index: Some(value.index()),
                typ: value.child_type().ok(),
            }
        }
    }
}

impl TryFrom<TrustCenterJoin> for ZigbeeMessage {
    type Error = u8;

    fn try_from(value: TrustCenterJoin) -> Result<Self, Self::Error> {
        Ok(match value.status()? {
            Update::StandardSecuritySecuredRejoin => Self::TrustCenterRejoin {
                id: value.new_node_eui64(),
                short_id: Some(value.new_node_id()),
                secure: true,
            },
            Update::StandardSecurityUnsecuredJoin => Self::TrustCenterJoin {
                id: value.new_node_eui64(),
                short_id: Some(value.new_node_id()),
            },
            Update::DeviceLeft => Self::TrustCenterLeave {
                id: value.new_node_eui64(),
                short_id: Some(value.new_node_id()),
            },
            Update::StandardSecurityUnsecuredRejoin => Self::TrustCenterRejoin {
                id: value.new_node_eui64(),
                short_id: Some(value.new_node_id()),
                secure: false,
            },
        })
    }
}

impl From<IncomingMessage> for ZigbeeMessage {
    fn from(value: IncomingMessage) -> Self {
        Self::IncomingMessage {
            source: Address::Short(value.sender()),
            data: value.into_message(),
        }
    }
}
