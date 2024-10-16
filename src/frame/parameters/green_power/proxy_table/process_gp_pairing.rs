use crate::ember::gp::Address;
use crate::ember::key::Data;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00C9;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    options: u32,
    addr: Address,
    comm_mode: u8,
    sink_network_address: u16,
    sink_group_id: u16,
    assigned_alias: u16,
    sink_ieee_address: [u8; 8],
    gpd_key: Data,
    gpd_security_frame_counter: u32,
    forwarding_radius: u8,
}

impl Command {
    #[must_use]
    pub const fn new(
        options: u32,
        addr: Address,
        comm_mode: u8,
        sink_network_address: u16,
        sink_group_id: u16,
        assigned_alias: u16,
        sink_ieee_address: [u8; 8],
        gpd_key: Data,
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    gp_pairing_added: bool,
}

impl Response {
    #[must_use]
    pub const fn gp_pairing_added(&self) -> bool {
        self.gp_pairing_added
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
