use std::time::Duration;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::gp::Address;
use crate::ember::Status;
use crate::error::value::Error;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Resolve;

const ID: u16 = 0x00C6;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    action: bool,
    use_cca: bool,
    addr: Address,
    gpd_command_id: u8,
    gpd_asdu: ByteSizedVec<u8>,
    gpep_handle: u8,
    gp_tx_queue_entry_lifetime_ms: u16,
}

impl Command {
    pub fn new(
        action: bool,
        use_cca: bool,
        addr: Address,
        gpd_command_id: u8,
        gpd_asdu: ByteSizedVec<u8>,
        gpep_handle: u8,
        gp_tx_queue_entry_lifetime: Duration,
    ) -> Result<Self, Error> {
        Ok(Self {
            action,
            use_cca,
            addr,
            gpd_command_id,
            gpd_asdu,
            gpep_handle,
            gp_tx_queue_entry_lifetime_ms: gp_tx_queue_entry_lifetime
                .as_millis()
                .try_into()
                .map_err(|_| Error::DurationTooLarge(gp_tx_queue_entry_lifetime))?,
        })
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
