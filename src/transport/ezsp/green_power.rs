use std::future::Future;

pub use proxy_table::ProxyTable;
pub use sink_table::SinkTable;

use crate::ember::gp::Address;
use crate::frame::parameters::green_power::{send, sink_commission, translation_table_clear};
use crate::types::ByteSizedVec;
use crate::{Error, Transport};

mod proxy_table;
mod sink_table;

/// The `GreenPower` trait provides an interface for the Green Power features.
pub trait GreenPower: ProxyTable + SinkTable {
    /// Adds/removes an entry from the GP Tx Queue.
    #[allow(clippy::too_many_arguments)]
    fn send(
        &mut self,
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime_millis: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Puts the GPS in commissioning mode.
    fn sink_commission(
        &mut self,
        options: u8,
        gpm_addr_for_security: u16,
        gpm_addr_for_pairing: u16,
        sink_endpoint: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Clears all entries within the translation table.
    fn translation_table_clear(&mut self) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> GreenPower for T
where
    T: ProxyTable + SinkTable + Transport,
{
    async fn send(
        &mut self,
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime_millis: u16,
    ) -> Result<(), Error> {
        send::Response::try_from(
            self.communicate(send::Command::new(
                action,
                use_cca,
                addr,
                gpd_command_id,
                gpd_asdu,
                gpep_handle,
                gp_tx_queue_entry_lifetime_millis,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn sink_commission(
        &mut self,
        options: u8,
        gpm_addr_for_security: u16,
        gpm_addr_for_pairing: u16,
        sink_endpoint: u8,
    ) -> Result<(), Error> {
        sink_commission::Response::try_from(
            self.communicate(sink_commission::Command::new(
                options,
                gpm_addr_for_security,
                gpm_addr_for_pairing,
                sink_endpoint,
            ))
            .await?,
        )?
        .try_into()
    }

    async fn translation_table_clear(&mut self) -> Result<(), Error> {
        Ok(translation_table_clear::Response::try_from(
            self.communicate(translation_table_clear::Command).await?,
        )
        .map(drop)?)
    }
}
