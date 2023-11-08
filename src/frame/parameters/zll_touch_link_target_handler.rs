
pub const ID: u16 = 0x00BB;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response{
    network_info: EmberZllNetwork,
}

impl Response {
    #[must_use]
    pub const fn new(network_info: EmberZllNetwork) -> Self {
        Self { network_info }
    }

    #[must_use]
    pub const fn network_info(&self) -> EmberZllNetwork {
        self.network_info
    }
}
