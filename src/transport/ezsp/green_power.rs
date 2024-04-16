use crate::ember::gp::Address;
use crate::types::ByteSizedVec;
use crate::{Error, Transport};
use std::future::Future;
use std::time::Duration;

pub trait GreenPower: Transport {
    /// Adds/removes an entry from the GP Tx Queue.
    #[allow(clippy::too_many_arguments)]
    fn d_gp_send(
        &self,
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime: Duration,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
