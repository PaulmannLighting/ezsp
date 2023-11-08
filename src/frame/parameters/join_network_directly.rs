
pub const ID: u16 = 0x003B;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    local_node_type: EmberNodeType,
    beacon: EmberBeaconData,
    radio_tx_power: int8_t,
    clear_beacons_after_network_up: bool,
}

impl Command {
    #[must_use]
    pub const fn new(local_node_type: EmberNodeType, beacon: EmberBeaconData, radio_tx_power: int8_t, clear_beacons_after_network_up: bool) -> Self {
        Self { local_node_type, beacon, radio_tx_power, clear_beacons_after_network_up }
    }

    #[must_use]
    pub const fn local_node_type(&self) -> EmberNodeType {
        self.local_node_type
    }


    #[must_use]
    pub const fn beacon(&self) -> EmberBeaconData {
        self.beacon
    }


    #[must_use]
    pub const fn radio_tx_power(&self) -> int8_t {
        self.radio_tx_power
    }


    #[must_use]
    pub const fn clear_beacons_after_network_up(&self) -> bool {
        self.clear_beacons_after_network_up
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
