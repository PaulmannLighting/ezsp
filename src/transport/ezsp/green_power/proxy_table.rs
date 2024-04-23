use crate::ember::gp::proxy::TableEntry;
use crate::ember::gp::Address;
use crate::ember::key::Data;
use crate::{Error, Transport};
use std::future::Future;

pub trait ProxyTable: Transport {
    /// Retrieves the proxy table entry stored at the passed index.
    fn get_entry(&self, proxy_index: u8) -> impl Future<Output = Result<TableEntry, Error>>;

    /// Finds the index of the passed address in the gp table.
    fn lookup(&self, addr: Address) -> impl Future<Output = Result<u8, Error>>;

    /// Update the GP Proxy table based on a GP pairing.
    fn process_gp_pairing(
        &self,
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
    ) -> impl Future<Output = Result<bool, Error>>;
}
