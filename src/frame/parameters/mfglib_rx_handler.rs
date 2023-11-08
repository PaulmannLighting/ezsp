pub const ID: u16 = 0x008E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    link_quality: u8,
    rssi: int8_t,
    packet_length: u8,
    packet_contents: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(
        link_quality: u8,
        rssi: int8_t,
        packet_length: u8,
        packet_contents: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            link_quality,
            rssi,
            packet_length,
            packet_contents,
        }
    }

    #[must_use]
    pub const fn link_quality(&self) -> u8 {
        self.link_quality
    }

    #[must_use]
    pub const fn rssi(&self) -> int8_t {
        self.rssi
    }

    #[must_use]
    pub const fn packet_length(&self) -> u8 {
        self.packet_length
    }

    #[must_use]
    pub const fn packet_contents(&self) -> ByteSizedVec<u8> {
        self.packet_contents
    }
}
