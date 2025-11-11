use core::future::Future;

use crate::ember::gp::Address;
use crate::ember::gp::proxy::TableEntry;
use crate::ember::key::Data;
use crate::error::Error;
use crate::frame::parameters::green_power::proxy_table::{get_entry, lookup, process_gp_pairing};
use crate::transport::Transport;

/// The `ProxyTable` trait provides an interface for the proxy table.
pub trait ProxyTable {
    /// Retrieves the proxy table entry stored at the passed index.
    fn get_entry(
        &mut self,
        proxy_index: u8,
    ) -> impl Future<Output = Result<TableEntry, Error>> + Send;

    /// Finds the index of the passed address in the gp table.
    fn lookup(&mut self, addr: Address) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Update the GP Proxy table based on a GP pairing.
    #[expect(clippy::too_many_arguments)]
    fn process_gp_pairing(
        &mut self,
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
    ) -> impl Future<Output = Result<bool, Error>> + Send;
}

impl<T> ProxyTable for T
where
    T: Transport,
{
    async fn get_entry(&mut self, proxy_index: u8) -> Result<TableEntry, Error> {
        self.communicate::<_, get_entry::Response>(get_entry::Command::new(proxy_index))
            .await?
            .try_into()
    }

    async fn lookup(&mut self, addr: Address) -> Result<u8, Error> {
        self.communicate::<_, lookup::Response>(lookup::Command::new(addr))
            .await
            .map(|response| response.index())
    }

    async fn process_gp_pairing(
        &mut self,
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
    ) -> Result<bool, Error> {
        self.communicate::<_, process_gp_pairing::Response>(process_gp_pairing::Command::new(
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
        ))
        .await
        .map(|response| response.gp_pairing_added())
    }
}
