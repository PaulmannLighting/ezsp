use std::time::Duration;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::gp::Address;
use crate::ember::Status;
use crate::error::ValueError;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{frame, Resolve};

const ID: u16 = 0x00C6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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
    ) -> Result<Self, ValueError> {
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
                .map_err(|_| ValueError::DurationTooLarge(gp_tx_queue_entry_lifetime))?,
        })
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
