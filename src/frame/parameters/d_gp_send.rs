
pub const ID: u16 = 0x00C6;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command{
    action: bool,
    use_cca: bool,
    addr: EmberGpAddress,
    gpd_command_id: u8,
    gpd_asdu_length: u8,
    gpd_asdu: uint8_t[],
    gpep_handle: u8,
    gp_tx_queue_entry_lifetime_ms: u16,
}

impl Command {
    #[must_use]
    pub const fn new(action: bool, use_cca: bool, addr: EmberGpAddress, gpd_command_id: u8, gpd_asdu_length: u8, gpd_asdu: uint8_t[], gpep_handle: u8, gp_tx_queue_entry_lifetime_ms: u16) -> Self {
        Self { action, use_cca, addr, gpd_command_id, gpd_asdu_length, gpd_asdu, gpep_handle, gp_tx_queue_entry_lifetime_ms }
    }

    #[must_use]
    pub const fn action(&self) -> bool {
        self.action
    }


    #[must_use]
    pub const fn use_cca(&self) -> bool {
        self.use_cca
    }


    #[must_use]
    pub const fn addr(&self) -> EmberGpAddress {
        self.addr
    }


    #[must_use]
    pub const fn gpd_command_id(&self) -> u8 {
        self.gpd_command_id
    }


    #[must_use]
    pub const fn gpd_asdu_length(&self) -> u8 {
        self.gpd_asdu_length
    }


    #[must_use]
    pub const fn gpd_asdu(&self) -> uint8_t[] {
        self.gpd_asdu
    }


    #[must_use]
    pub const fn gpep_handle(&self) -> u8 {
        self.gpep_handle
    }


    #[must_use]
    pub const fn gp_tx_queue_entry_lifetime_ms(&self) -> u16 {
        self.gp_tx_queue_entry_lifetime_ms
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
