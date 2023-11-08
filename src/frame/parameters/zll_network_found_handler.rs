
pub const ID: u16 = 0x00B6;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    network_info: EmberZllNetwork,
    is_device_info_null: bool,
    device_info: EmberZllDeviceInfoRecord,
    last_hop_lqi: u8,
    last_hop_rssi: int8_t,
}

impl Response {
    #[must_use]
    pub const fn new(network_info: EmberZllNetwork, is_device_info_null: bool, device_info: EmberZllDeviceInfoRecord, last_hop_lqi: u8, last_hop_rssi: int8_t) -> Self {
        Self { network_info, is_device_info_null, device_info, last_hop_lqi, last_hop_rssi }
    }

    #[must_use]
    pub const fn network_info(&self) -> EmberZllNetwork {
        self.network_info
    }


    #[must_use]
    pub const fn is_device_info_null(&self) -> bool {
        self.is_device_info_null
    }


    #[must_use]
    pub const fn device_info(&self) -> EmberZllDeviceInfoRecord {
        self.device_info
    }


    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }


    #[must_use]
    pub const fn last_hop_rssi(&self) -> int8_t {
        self.last_hop_rssi
    }
}
