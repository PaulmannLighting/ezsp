
pub const ID: u16 = 0x0046;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    filter_index_match: u8,
    legacy_passthrough_type: EmberMacPassthroughType,
    last_hop_lqi: u8,
    last_hop_rssi: int8s,
    message_length: u8,
    message_contents: uint8_t[],
}

impl Response {
    #[must_use]
    pub const fn new(filter_index_match: u8, legacy_passthrough_type: EmberMacPassthroughType, last_hop_lqi: u8, last_hop_rssi: int8s, message_length: u8, message_contents: uint8_t[]) -> Self {
        Self { filter_index_match, legacy_passthrough_type, last_hop_lqi, last_hop_rssi, message_length, message_contents }
    }

    #[must_use]
    pub const fn filter_index_match(&self) -> u8 {
        self.filter_index_match
    }


    #[must_use]
    pub const fn legacy_passthrough_type(&self) -> EmberMacPassthroughType {
        self.legacy_passthrough_type
    }


    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }


    #[must_use]
    pub const fn last_hop_rssi(&self) -> int8s {
        self.last_hop_rssi
    }


    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }


    #[must_use]
    pub const fn message_contents(&self) -> uint8_t[] {
        self.message_contents
    }
}
