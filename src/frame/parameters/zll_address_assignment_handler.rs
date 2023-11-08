
pub const ID: u16 = 0x00B8;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    address_info: EmberZllAddressAssignment,
    last_hop_lqi: u8,
    last_hop_rssi: int8_t,
}

impl Response {
    #[must_use]
    pub const fn new(address_info: EmberZllAddressAssignment, last_hop_lqi: u8, last_hop_rssi: int8_t) -> Self {
        Self { address_info, last_hop_lqi, last_hop_rssi }
    }

    #[must_use]
    pub const fn address_info(&self) -> EmberZllAddressAssignment {
        self.address_info
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
