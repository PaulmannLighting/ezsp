//! Beacon data structures.

use le_stream::derive::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;

use crate::ember::types::PanId;

/// Beacon data structure.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Data {
    channel: u8,
    lqi: u8,
    rssi: i8,
    depth: u8,
    nwk_update_id: u8,
    power: i8,
    parent_priority: i8,
    pan_id: PanId,
    extended_pan_id: MacAddr8,
    sender: u16,
    enhanced: bool,
    permit_join: bool,
    has_capacity: bool,
}

impl Data {
    /// Create a new beacon data structure.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        channel: u8,
        lqi: u8,
        rssi: i8,
        depth: u8,
        nwk_update_id: u8,
        power: i8,
        parent_priority: i8,
        pan_id: PanId,
        extended_pan_id: MacAddr8,
        sender: u16,
        enhanced: bool,
        permit_join: bool,
        has_capacity: bool,
    ) -> Self {
        Self {
            channel,
            lqi,
            rssi,
            depth,
            nwk_update_id,
            power,
            parent_priority,
            pan_id,
            extended_pan_id,
            sender,
            enhanced,
            permit_join,
            has_capacity,
        }
    }

    /// Return the channel of the received beacon.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// Return the LQI of the received beacon.
    #[must_use]
    pub const fn lqi(&self) -> u8 {
        self.lqi
    }

    /// Return the RSSI of the received beacon.
    #[must_use]
    pub const fn rssi(&self) -> i8 {
        self.rssi
    }

    /// Return the depth of the received beacon.
    #[must_use]
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    /// Return the network update ID of the received beacon.
    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }

    /// Return the power level of the received beacon.
    ///
    /// This field is valid only if the beacon is an enhanced beacon.
    #[must_use]
    pub const fn power(&self) -> Option<i8> {
        if self.enhanced {
            Some(self.power)
        } else {
            None
        }
    }

    /// Return the TC connectivity and long uptime from capacity field.
    #[must_use]
    pub const fn parent_priority(&self) -> i8 {
        self.parent_priority
    }

    /// Return the PAN ID of the received beacon.
    #[must_use]
    pub const fn pan_id(&self) -> PanId {
        self.pan_id
    }

    /// Return the extended PAN ID of the received beacon.
    #[must_use]
    pub const fn extended_pan_id(&self) -> MacAddr8 {
        self.extended_pan_id
    }

    /// Return the sender of the received beacon.
    #[must_use]
    pub const fn sender(&self) -> u16 {
        self.sender
    }

    /// Return whether the beacon is enhanced.
    #[must_use]
    pub const fn enhanced(&self) -> bool {
        self.enhanced
    }

    /// Return whether the beacon is advertising permit join.
    #[must_use]
    pub const fn permit_join(&self) -> bool {
        self.permit_join
    }

    /// Return whether the beacon is advertising capacity.
    #[must_use]
    pub const fn has_capacity(&self) -> bool {
        self.has_capacity
    }
}

/// The parameters related to beacon prioritization.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct ClassificationParams {
    min_rssi_for_receiving_pkts: i8,
    beacon_classification_mask: u16,
}

impl ClassificationParams {
    /// Create new classification parameters.
    #[must_use]
    pub const fn new(min_rssi_for_receiving_pkts: i8, beacon_classification_mask: u16) -> Self {
        Self {
            min_rssi_for_receiving_pkts,
            beacon_classification_mask,
        }
    }

    /// Return the minimum RSSI value for receiving packets that is used in some beacon
    /// prioritization algorithms.
    #[must_use]
    pub const fn min_rssi_for_receiving_pkts(&self) -> i8 {
        self.min_rssi_for_receiving_pkts
    }

    /// Return the beacon classification mask that identifies which beacon prioritization algorithm
    /// to pick and defines the relevant parameters.
    #[must_use]
    pub const fn beacon_classification_mask(&self) -> u16 {
        self.beacon_classification_mask
    }
}

/// Defines an iterator that is used to loop over cached beacons.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Iterator {
    beacon: Data,
    index: u8,
}

impl Iterator {
    /// Create a new beacon iterator.
    #[must_use]
    pub const fn new(beacon: Data, index: u8) -> Self {
        Self { beacon, index }
    }

    /// Return the retrieved beacon.
    #[must_use]
    pub const fn beacon(&self) -> &Data {
        &self.beacon
    }

    /// Return the index of the retrieved beacon.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}
