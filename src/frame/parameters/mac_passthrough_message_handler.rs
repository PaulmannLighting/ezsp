pub const ID: u16 = 0x0097;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    message_type: EmberMacPassthroughType,
    last_hop_lqi: u8,
    last_hop_rssi: int8s,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(
        message_type: EmberMacPassthroughType,
        last_hop_lqi: u8,
        last_hop_rssi: int8s,
        message_length: u8,
        message_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            message_type,
            last_hop_lqi,
            last_hop_rssi,
            message_length,
            message_contents,
        }
    }

    #[must_use]
    pub const fn message_type(&self) -> EmberMacPassthroughType {
        self.message_type
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
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }
}
