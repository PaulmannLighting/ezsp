use std::future::Future;
use std::time::Duration;

pub use proxy_table::ProxyTable;
pub use sink_table::SinkTable;

use crate::ember::gp::Address;
use crate::types::ByteSizedVec;
use crate::Error;

mod proxy_table;
mod sink_table;

pub trait GreenPower: ProxyTable + SinkTable {
    /// Adds/removes an entry from the GP Tx Queue.
    #[allow(clippy::too_many_arguments)]
    fn send(
        &self,
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime: Duration,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Puts the GPS in commissioning mode.
    fn sink_commission(
        &self,
        options: u8,
        gpm_addr_for_security: u16,
        gpm_addr_for_pairing: u16,
        sink_endpoint: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clears all entries within the translation table.
    fn translation_table_clear(&self) -> impl Future<Output = Result<(), Error>> + Send;
}
