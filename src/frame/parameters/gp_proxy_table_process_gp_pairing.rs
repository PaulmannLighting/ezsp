pub const ID: u16 = 0x00C9;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    options: u32,
    addr: EmberGpAddress,
    comm_mode: u8,
    sink_network_address: u16,
    sink_group_id: u16,
    assigned_alias: u16,
    sink_ieee_address: [u8; 8],
    gpd_key: EmberKeyData,
    gpd_security_frame_counter: u32,
    forwarding_radius: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        options: u32,
        addr: EmberGpAddress,
        comm_mode: u8,
        sink_network_address: u16,
        sink_group_id: u16,
        assigned_alias: u16,
        sink_ieee_address: [u8; 8],
        gpd_key: EmberKeyData,
        gpd_security_frame_counter: u32,
        forwarding_radius: u8,
    ) -> Self {
        Self {
            options,
            addr,
            comm_mode,
            sink_network_address,
            sink_group_id,
            assigned_alias,
            sink_ieee_address,
            gpd_key,
            gpd_security_frame_counter,
            forwarding_radius,
        }
    }

    #[must_use]
    pub const fn options(&self) -> u32 {
        self.options
    }

    #[must_use]
    pub const fn addr(&self) -> EmberGpAddress {
        self.addr
    }

    #[must_use]
    pub const fn comm_mode(&self) -> u8 {
        self.comm_mode
    }

    #[must_use]
    pub const fn sink_network_address(&self) -> u16 {
        self.sink_network_address
    }

    #[must_use]
    pub const fn sink_group_id(&self) -> u16 {
        self.sink_group_id
    }

    #[must_use]
    pub const fn assigned_alias(&self) -> u16 {
        self.assigned_alias
    }

    #[must_use]
    pub const fn sink_ieee_address(&self) -> [u8; 8] {
        self.sink_ieee_address
    }

    #[must_use]
    pub const fn gpd_key(&self) -> EmberKeyData {
        self.gpd_key
    }

    #[must_use]
    pub const fn gpd_security_frame_counter(&self) -> u32 {
        self.gpd_security_frame_counter
    }

    #[must_use]
    pub const fn forwarding_radius(&self) -> u8 {
        self.forwarding_radius
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    gp_pairing_added: bool,
}

impl Response {
    #[must_use]
    pub const fn new(gp_pairing_added: bool) -> Self {
        Self { gp_pairing_added }
    }

    #[must_use]
    pub const fn gp_pairing_added(&self) -> bool {
        self.gp_pairing_added
    }
}
